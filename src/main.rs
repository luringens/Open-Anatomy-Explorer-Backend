#![feature(decl_macro)]

use rocket::routes;
use rocket_contrib::serve::StaticFiles;

mod labelpoint;
mod models;
mod quiz;
mod util;

fn main() {
    dotenv::dotenv().ok();
    const REQUIRED_ENV: [&'static str; 3] = ["QUIZ_DATA_DIR", "LABEL_DATA_DIR", "MODELS_DIR"];
    for var in REQUIRED_ENV.iter() {
        if let Err(_) = std::env::var(var) {
            eprintln!("{} not found in environment variables.", var);
            return;
        }
    }

    rocket::ignite()
        .mount("/Quiz", routes![quiz::load, quiz::create, quiz::delete])
        .mount(
            "/LabelPoints",
            routes![labelpoint::create, labelpoint::load, labelpoint::delete],
        )
        .mount(
            "/models",
            StaticFiles::from(std::env::var("MODELS_DIR").unwrap()).rank(-1),
        )
        .mount("/models", routes![models_index])
        .attach(Cors {})
        .launch();
}

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

struct Cors {}

impl rocket::fairing::Fairing for Cors {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "CORS header fairing",
            kind: rocket::fairing::Kind::Response,
        }
    }

    fn on_response(&self, _: &rocket::Request, response: &mut rocket::Response) {
        response.adjoin_raw_header("Access-Control-Allow-Origin", "*");
    }
}
