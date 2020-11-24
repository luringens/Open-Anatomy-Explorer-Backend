use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelPoint {
    pub id: i32,
    pub color: String,
    pub name: String,
    pub model: String,
    pub vertices: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}
