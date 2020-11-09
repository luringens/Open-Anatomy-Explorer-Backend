#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::{json::Json, serve::StaticFiles, uuid::Uuid};
use std::{env, error::Error};

mod models;
use models::*;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                load_quiz,
                create_quiz,
                delete_quiz,
                create_label_point,
                load_label_point,
                delete_label_point,
            ],
        )
        .mount("/models", StaticFiles::from("/models"))
        .launch();
}

#[get("/LoadQuiz/<uuid>")]
fn load_quiz(uuid: Uuid) -> Result<Json<Quiz>, Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    let data = std::fs::read(format!("{}/{}.json", data_dir, &uuid))?;
    let string = std::str::from_utf8(&data)?;
    let result = serde_json::from_str(string)?;
    Ok(Json(result))
}

#[post("/CreateQuiz?<uuid>", format = "json", data = "<data>")]
fn create_quiz(uuid: Option<Uuid>, data: Json<Quiz>) -> Result<Json<String>, Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    let id = uuid.unwrap_or_else(create_uuid);
    let data = serde_json::to_string(&data.into_inner())?;
    std::fs::write(format!("{}/{}.json", data_dir, id), data)?;
    Ok(Json(id.to_string()))
}

#[post("/DeleteQuiz/<uuid>")]
fn delete_quiz(uuid: Uuid) -> Result<(), Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    std::fs::remove_file(format!("{}/{}.json", data_dir, &uuid))?;
    Ok(())
}

#[post("/CreateLabelPoint?<uuid>", format = "json", data = "<data>")]
fn create_label_point(
    uuid: Option<Uuid>,
    data: Json<Vec<LabelPoint>>,
) -> Result<Json<String>, Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    let id = uuid.unwrap_or_else(create_uuid);
    let data = serde_json::to_string(&data.into_inner())?;
    std::fs::write(format!("{}/{}.json", data_dir, id), data)?;
    Ok(Json(id.to_string()))
}

#[get("/LoadLabelPoint/<uuid>")]
fn load_label_point(uuid: Uuid) -> Result<Json<Vec<LabelPoint>>, Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    let data = std::fs::read(format!("{}/{}.json", data_dir, &uuid))?;
    let string = std::str::from_utf8(&data)?;
    let result = serde_json::from_str(string)?;
    Ok(Json(result))
}

#[post("/DeleteLabelPoint/<uuid>")]
fn delete_label_point(uuid: Uuid) -> Result<(), Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    std::fs::remove_file(format!("{}/{}.json", data_dir, &uuid))?;
    Ok(())
}

fn create_uuid() -> Uuid {
    uuid::Uuid::new_v4().to_string().parse::<Uuid>().unwrap()
}

// use crate::label::database::*;
// use crate::label::LabelPoint;
// use crate::quiz::database::*;
// use crate::quiz::Quiz;

// mod label;
// mod quiz;

// pub struct State {
//     label_db: Addr<LabelDbExecutor>,
//     quiz_db: Addr<QuizDbExecutor>,
// }

// fn files(
//     dir: &actix_files::Directory,
//     req: &actix_web::HttpRequest,
// ) -> Result<actix_web::dev::ServiceResponse, std::io::Error> {
//     let mut names: Vec<String> = Vec::new();
//     for file in std::fs::read_dir(&dir.path)? {
//         names.push(
//             file?
//                 .file_name()
//                 .into_string()
//                 .unwrap_or_else(|e| format!("{:?}", e)),
//         );
//     }

//     Ok(actix_web::dev::ServiceResponse::new(
//         req.clone(),
//         actix_web::HttpResponse::Ok().json(names),
//     ))
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     dotenv().ok();
//     env_logger::init();

//     let label_dir = env::var("LABEL_DATA_DIR").expect("LABEL_DATA_DIR not set");
//     match std::fs::create_dir(&label_dir) {
//         Ok(_) => {}
//         Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
//         _ => panic!("Could not find or create label data dir!"),
//     }
//     let quiz_dir = env::var("QUIZ_DATA_DIR").expect("QUIZ_DATA_DIR not set");
//     match std::fs::create_dir(&quiz_dir) {
//         Ok(_) => {}
//         Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
//         _ => panic!("Could not find or create quiz data dir!"),
//     }

//     let label_addr = SyncArbiter::start(1, LabelDbExecutor::new);
//     let quiz_addr = SyncArbiter::start(1, QuizDbExecutor::new);
//     let mut listenfd = ListenFd::from_env();
//     let mut server = HttpServer::new(move || {
//         App::new()
//             .wrap(Logger::default())
//             .wrap(Logger::new("%a %{User-Agent}i"))
//             .wrap(actix_cors::Cors::default())
//             .data(State {
//                 label_db: label_addr.clone(),
//                 quiz_db: quiz_addr.clone(),
//             })
//             .configure(label::init_routes)
//             .configure(quiz::init_routes)
//             .service(
//                 fs::Files::new("/models", "./models")
//                     .show_files_listing()
//                     .files_listing_renderer(files),
//             )
//             .app_data(web::Json::<LabelPoint>::configure(|cfg| cfg.limit(2097152)))
//             .app_data(web::Json::<Quiz>::configure(|cfg| cfg.limit(2097152)))
//     });

//     server = match listenfd.take_tcp_listener(0)? {
//         Some(listener) => server.listen(listener)?,
//         None => {
//             let host = env::var("HOST").expect("Host not set");
//             let port = env::var("PORT").expect("Port not set");
//             server.bind(format!("{}:{}", host, port))?
//         }
//     };

//     info!("Starting server");
//     server.run().await
// }
