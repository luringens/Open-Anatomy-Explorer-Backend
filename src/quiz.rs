use crate::util;
use crate::{models::Quiz, util::json_path};
use rocket::{delete, get, post, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use std::{env, error::Error};

#[get("/<uuid>")]
pub fn load(uuid: Uuid) -> Result<Json<Quiz>, Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    let data = std::fs::read(json_path(&data_dir, &uuid.to_string()))?;
    let string = std::str::from_utf8(&data)?;
    let result = serde_json::from_str(string)?;
    Ok(Json(result))
}

#[post("/", format = "json", data = "<data>")]
pub fn create(data: Json<Quiz>) -> Result<Json<String>, Box<dyn Error>> {
    put(util::create_uuid(), data)
}

#[put("/<uuid>", format = "json", data = "<data>")]
pub fn put(uuid: Uuid, data: Json<Quiz>) -> Result<Json<String>, Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    let data = serde_json::to_string(&data.into_inner())?;
    std::fs::write(json_path(&data_dir, &uuid.to_string()), data)?;
    Ok(Json(uuid.to_string()))
}

#[delete("/<uuid>")]
pub fn delete(uuid: Uuid) -> Result<(), Box<dyn Error>> {
    let data_dir = env::var("QUIZ_DATA_DIR").unwrap();
    std::fs::remove_file(json_path(&data_dir, &uuid.to_string()))?;
    Ok(())
}
