use crate::database::*;
use crate::label::*;
use crate::State;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::warn;
use uuid::Uuid;

#[get("/LabelPoints/{uid}")]
async fn find(state: web::Data<State>, uid: web::Path<Uuid>) -> impl Responder {
    let id = uid.into_inner();
    state
        .db
        .send(LoadLabelPoint { id })
        .await
        .and_then(|res| match res {
            Ok(label) => Ok(HttpResponse::Ok().json(label)),
            Err(e) => {
                warn!("{}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
}

#[post("/LabelPoints")]
async fn create(state: web::Data<State>, data: web::Json<Vec<LabelPoint>>) -> impl Responder {
    state
        .db
        .send(CreateLabelPoint {
            data: data.into_inner(),
            uuid: None,
        })
        .await
        .and_then(|res| match res {
            Ok(id) => Ok(HttpResponse::Created().json(id)),
            Err(e) => {
                warn!("{}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
}

#[put("/LabelPoints/{id}")]
async fn update(
    state: web::Data<State>,
    data: web::Json<Vec<LabelPoint>>,
    uid: web::Path<Uuid>,
) -> impl Responder {
    state
        .db
        .send(CreateLabelPoint {
            data: data.into_inner(),
            uuid: Some(uid.into_inner()),
        })
        .await
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok()),
            Err(e) => {
                warn!("{}", e);
                Ok(HttpResponse::InternalServerError())
            }
        })
}

#[delete("/LabelPoints/{id}")]
async fn delete(state: web::Data<State>, uid: web::Path<Uuid>) -> impl Responder {
    let id = uid.into_inner();
    state
        .db
        .send(DeleteLabelPoint { id })
        .await
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok()),
            Err(e) => {
                warn!("{}", e);
                Ok(HttpResponse::InternalServerError())
            }
        })
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
