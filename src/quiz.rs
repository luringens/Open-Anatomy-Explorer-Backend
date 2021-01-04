use crate::util;
use crate::util::json_path;
use rocket::{delete, get, post, put};
use rocket_contrib::{json::Json, uuid::Uuid};
use serde::{Deserialize, Serialize};
use std::{env, error::Error};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub questions: Vec<Question>,
    pub model: String,
    pub label_id: String,
    pub shuffle: bool,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Debug)]
pub struct Question {
    pub question_type: u8,
    pub id: u32,
    pub text_prompt: String,
    pub text_answer: Option<String>,
    pub label_id: u32,
    pub show_regions: Option<bool>,
}

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
