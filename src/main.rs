// src/main.rs
use actix::prelude::*;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use log::info;
use std::env;

use crate::label::database::*;
use crate::quiz::database::*;

mod label;
mod quiz;

pub struct State {
    label_db: Addr<LabelDbExecutor>,
    quiz_db: Addr<QuizDbExecutor>,
}

fn files(
    dir: &actix_files::Directory,
    req: &actix_web::HttpRequest,
) -> Result<actix_web::dev::ServiceResponse, std::io::Error> {
    let mut names: Vec<String> = Vec::new();
    for file in std::fs::read_dir(&dir.path)? {
        names.push(
            file?
                .file_name()
                .into_string()
                .unwrap_or_else(|e| format!("{:?}", e)),
        );
    }

    Ok(actix_web::dev::ServiceResponse::new(
        req.clone(),
        actix_web::HttpResponse::Ok().json(names),
    ))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let label_dir = env::var("LABEL_DATA_DIR").expect("LABEL_DATA_DIR not set");
    match std::fs::create_dir(&label_dir) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        _ => panic!("Could not find or create label data dir!"),
    }
    let quiz_dir = env::var("QUIZ_DATA_DIR").expect("QUIZ_DATA_DIR not set");
    match std::fs::create_dir(&quiz_dir) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        _ => panic!("Could not find or create quiz data dir!"),
    }

    let label_addr = SyncArbiter::start(1, LabelDbExecutor::new);
    let quiz_addr = SyncArbiter::start(1, QuizDbExecutor::new);
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(actix_cors::Cors::default())
            .data(State {
                label_db: label_addr.clone(),
                quiz_db: quiz_addr.clone(),
            })
            .configure(label::init_routes)
            .configure(quiz::init_routes)
            .service(
                fs::Files::new("/models", "./models")
                    .show_files_listing()
                    .files_listing_renderer(files),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await
}
