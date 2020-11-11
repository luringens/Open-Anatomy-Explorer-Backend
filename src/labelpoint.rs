use crate::util;
use crate::{models::LabelPoint, util::json_path};
use rocket::{delete, get, post, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use std::{env, error::Error};

#[post("/", format = "json", data = "<data>")]
pub fn create(data: Json<Vec<LabelPoint>>) -> Result<Json<String>, Box<dyn Error>> {
    put(util::create_uuid(), data)
}

#[put("/<uuid>", format = "json", data = "<data>")]
pub fn put(uuid: Uuid, data: Json<Vec<LabelPoint>>) -> Result<Json<String>, Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    let data = serde_json::to_string(&data.into_inner())?;
    std::fs::write(dbg!(json_path(&data_dir, &uuid.to_string())), data)?;
    Ok(Json(uuid.to_string()))
}

#[get("/<uuid>")]
pub fn load(uuid: Uuid) -> Result<Json<Vec<LabelPoint>>, Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    let data = std::fs::read(json_path(&data_dir, &uuid.to_string()))?;
    let string = std::str::from_utf8(&data)?;
    let result = serde_json::from_str(string)?;
    Ok(Json(result))
}

#[delete("/<uuid>")]
pub fn delete(uuid: Uuid) -> Result<(), Box<dyn Error>> {
    let data_dir = env::var("LABEL_DATA_DIR").unwrap();
    std::fs::remove_file(json_path(&data_dir, &uuid.to_string()))?;
    Ok(())
}
