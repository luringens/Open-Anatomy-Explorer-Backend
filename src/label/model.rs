use serde::{Deserialize, Serialize};

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
