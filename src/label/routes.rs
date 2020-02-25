// src/LabelPoint/routes.rs
use crate::label::*;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/LabelPoints")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        LabelPoint {
            id: 1,
            position: Vector2 { x: 0.0, y: 0.0 },
            color: "#FF0000".to_string(),
        },
        LabelPoint {
            id: 2,
            position: Vector2 { x: 0.0, y: 0.0 },
            color: "#FF0000".to_string(),
        },
    ])
}

#[get("/LabelPoints/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(LabelPoint {
        id: 1,
        position: Vector2 { x: 0.0, y: 0.0 },
        color: "#FF0000".to_string(),
    })
}

#[post("/LabelPoints")]
async fn create(labelpoint: web::Json<LabelPoint>) -> impl Responder {
    HttpResponse::Created().json(labelpoint.into_inner())
}

#[put("/LabelPoints/{id}")]
async fn update(labelpoint: web::Json<LabelPoint>) -> impl Responder {
    HttpResponse::Ok().json(labelpoint.into_inner())
}

#[delete("/LabelPoints/{id}")]
async fn delete() -> impl Responder {
    HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
