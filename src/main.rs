#![feature(decl_macro)]

// Bulk macro imports for the schema module.
#[macro_use]
extern crate diesel;

use rocket::routes;
use rocket_contrib::{database, serve::StaticFiles};
mod labels;
mod models;
mod quiz;
mod schema;
mod users;
mod util;

#[database("sqlite_db")]
pub struct MainDbConn(diesel::SqliteConnection);

fn main() {
    // Load environment variable from `.env` if present
    dotenv::dotenv().ok();
    sodiumoxide::init().expect("Failed to initialize sodiumoxide`.");

    // Confirm all required environment variables are present and the directories exist.
    {
        const REQUIRED_ENV: [&'static str; 3] = ["QUIZ_DATA_DIR", "LABEL_DATA_DIR", "MODELS_DIR"];
        let mut missing_env = false;
        for env in REQUIRED_ENV.iter() {
            if let Ok(path) = std::env::var(env) {
                if let Err(e) = std::fs::create_dir_all(path) {
                    missing_env = true;
                    eprintln!("Path for '{}' could not be created: {:?}", env, e);
                }
            } else {
                missing_env = true;
                eprintln!("Missing environment variable '{}'", env);
            }
        }
        if missing_env {
            return;
        }
    }

    // Set up CORS as this API will be called from other pages.
    let mut allowed_origins = vec!["http://localhost:*".to_owned()];
    if let Ok(cors) = std::env::var("CORS") {
        allowed_origins.push(cors);
    }
    let cors = rocket_cors::CorsOptions::default()
        .allowed_origins(rocket_cors::AllowedOrigins::some_regex(&allowed_origins))
        .to_cors()
        .expect("Failed to initialize CORS.");

    // Mount paths and cors fairing and launch the application.
    rocket::ignite()
        .attach(MainDbConn::fairing())
        .mount(
            "/quiz",
            routes![quiz::load, quiz::create, quiz::delete, quiz::put],
        )
        .mount(
            "/labels",
            routes![labels::create, labels::load, labels::put, labels::delete],
        )
        .mount(
            "/models",
            StaticFiles::from(std::env::var("MODELS_DIR").unwrap()).rank(isize::max_value()),
        )
        .mount("/models", routes![models_index])
        .mount("/users", routes![users::login, users::create])
        .attach(cors)
        .launch();
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
