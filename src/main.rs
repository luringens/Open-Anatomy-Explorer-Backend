#![feature(decl_macro)]

use rocket::routes;
use rocket_contrib::serve::StaticFiles;

mod labelpoint;
mod models;
mod quiz;
mod util;

fn main() {
    dotenv::dotenv().ok();
    const REQUIRED_ENV: [&'static str; 2] = ["QUIZ_DATA_DIR", "LABEL_DATA_DIR"];
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
        .mount("/models", StaticFiles::from("/models"))
        .launch();
}
