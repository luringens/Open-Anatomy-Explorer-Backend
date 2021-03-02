#![feature(decl_macro, never_type)]

// Bulk macro imports for the schema module.
#[macro_use]
extern crate diesel;

use rocket::routes;
use rocket_contrib::{database, serve::StaticFiles};
mod authentication;
mod labels;
mod models;
mod modelstorage;
mod quiz;
mod schema;
mod users;
mod util;

#[database("sqlite_db")]
pub struct MainDbConn(diesel::SqliteConnection);

fn main() {
    // Load environment variable from `.env` if present
    dotenv::dotenv().ok();

    // Initialize cryptography crate.
    sodiumoxide::init().expect("Failed to initialize sodiumoxide`.");

    // Confirm the required environment variable are present and the directory exists.
    if let Ok(path) = std::env::var("MODELS_DIR") {
        if let Err(e) = std::fs::create_dir_all(path) {
            eprintln!("Path for 'MODELS_DIR' could not be created: {:?}", e);
            return;
        }
    } else {
        eprintln!("Missing environment variable 'MODELS_DIR'");
        return;
    }

    // Set up CORS as this API will be called from other pages.
    let mut allowed_origins = vec![
        r"^https?://localhost:(\d+){1,6}$".to_owned(),
        r"^https?://oah\.stiansoltvedt\.com$".to_owned(),
    ];
    if let Ok(cors) = std::env::var("CORS") {
        allowed_origins.push(cors);
    }
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::some_regex(&allowed_origins))
        .allow_credentials(true)
        .to_cors()
        .expect("Failed to initialize CORS.");

    // Mount paths and cors fairing and launch the application.
    let mut rocket = rocket::ignite()
        .attach(MainDbConn::fairing())
        .attach(cors)
        .mount(
            "/quiz",
            routes![quiz::load, quiz::create, quiz::delete, quiz::put],
        )
        .mount(
            "/labels",
            routes![
                labels::create,
                labels::load,
                labels::put,
                labels::delete,
                labels::load_by_uuid,
            ],
        )
        .mount(
            "/models",
            StaticFiles::from(std::env::var("MODELS_DIR").unwrap()).rank(isize::max_value() - 1),
        )
        .mount("/models", routes![models_index])
        .mount(
            "/modelstorage",
            routes![
                modelstorage::upload,
                modelstorage::upload_material,
                modelstorage::upload_texture,
                modelstorage::lookup,
                modelstorage::list,
            ],
        )
        .mount(
            "/users/quizzes",
            routes![
                users::quizzes::get,
                users::quizzes::add,
                users::quizzes::delete,
            ],
        )
        .mount(
            "/users/labelsets",
            routes![
                users::labelsets::get,
                users::labelsets::add,
                users::labelsets::delete,
            ],
        )
        .mount(
            "/users",
            routes![
                users::login,
                users::logout,
                users::create,
                users::is_admin,
                users::is_not_admin,
                users::is_moderator,
                users::is_not_moderator,
                users::refresh_session_user,
                users::refresh_session_loggedout,
            ],
        );

    // Attempt to mount the accompanying website as a static directory if present.
    if let Ok(path) = std::env::var("SITE_DIR") {
        if let Err(e) = std::fs::create_dir_all(&path) {
            eprintln!("Path for 'MODELS_DIR' could not be created: {:?}", e);
        } else {
            rocket = rocket.mount("/", StaticFiles::from(path).rank(isize::max_value()));
        }
    }

    rocket.launch();
}

/// Overrides models index with a json list of models.
#[rocket::get("/")]
pub fn models_index() -> Result<rocket_contrib::json::Json<Vec<String>>, Box<dyn std::error::Error>>
{
    let data_dir = std::env::var("MODELS_DIR").unwrap();
    let paths: Vec<_> = std::fs::read_dir(data_dir)?
        .into_iter()
        .filter_map(|f| f.ok().and_then(|d| d.file_name().into_string().ok()))
        .collect();

    Ok(rocket_contrib::json::Json(paths))
}
