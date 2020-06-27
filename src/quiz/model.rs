use serde::{Deserialize, Serialize};

#[serde(rename_all = "camelCase")] 
#[derive(Serialize, Deserialize, Debug)]
pub struct Quiz {
    pub questions: Vec<Question>,
    pub model: String,
    pub label_id: String,
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
