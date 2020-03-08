// src/main.rs
use actix::prelude::*;
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let cors = env::var("CORS_ACCEPT").expect("CORS not set");

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
            .wrap(actix_cors::Cors::new().allowed_origin(&cors).finish())
            .data(State { db: addr.clone() })
            .configure(label::init_routes)
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
