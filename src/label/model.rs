use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelPoint {
    pub id: i32,
    pub position: Vector2,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LabelRegion {
    pub id: i32,
    pub position: Vector3,
    pub color: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
