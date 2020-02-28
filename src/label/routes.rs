// src/LabelPoint/routes.rs
use crate::database::*;
use crate::label::*;
use crate::State;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;

#[get("/LabelPoints/{id}")]
async fn find(data: web::Data<State>) -> impl Responder {
    data.db
        .send(LoadLabelPoint { id: "123" })
        .from_err()
        .and_then(|res| match res {
            Ok(label) => Ok(HttpResponse::Ok().json(label)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        })
        .responder()
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
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
