// src/main.rs
use actix::prelude::*;
use actix_files as fs;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use database::*;
use dotenv::dotenv;
use listenfd::ListenFd;
use log::info;
use std::env;

mod database;
mod label;

struct State {
    db: Addr<DbExecutor>,
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

    let dir = env::var("DATA_DIR").expect("DATA_DIR not set");
    match std::fs::create_dir(&dir) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
        _ => panic!("Could not find or create data dir!"),
    }

    let addr = SyncArbiter::start(1, DbExecutor::new);
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(actix_cors::Cors::default())
            .data(State { db: addr.clone() })
            .configure(label::init_routes)
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
