use crate::models::Quiz;
use crate::util;
use rocket::{delete, get, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use std::{env, error::Error};

#[get("/<uuid>")]
pub fn load(uuid: Uuid) -> Result<Json<Quiz>, Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    let data = std::fs::read(format!("{}/{}.json", data_dir, &uuid))?;
    let string = std::str::from_utf8(&data)?;
    let result = serde_json::from_str(string)?;
    Ok(Json(result))
}

#[put("/?<uuid>", format = "json", data = "<data>")]
pub fn create(uuid: Option<Uuid>, data: Json<Quiz>) -> Result<Json<String>, Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    let id = uuid.unwrap_or_else(util::create_uuid);
    let data = serde_json::to_string(&data.into_inner())?;
    std::fs::write(format!("{}/{}.json", data_dir, id), data)?;
    Ok(Json(id.to_string()))
}

#[delete("/<uuid>")]
pub fn delete(uuid: Uuid) -> Result<(), Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    std::fs::remove_file(format!("{}/{}.json", data_dir, &uuid))?;
    Ok(())
}
