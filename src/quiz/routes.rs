use crate::quiz::database::*;
use crate::quiz::*;
use crate::State;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::warn;
use uuid::Uuid;

#[get("/Quiz/{uid}")]
async fn find(state: web::Data<State>, uid: web::Path<Uuid>) -> impl Responder {
    let id = uid.into_inner();
    state
        .quiz_db
        .send(LoadQuiz { id })
        .await
        .and_then(|res| match res {
            Ok(label) => Ok(HttpResponse::Ok().json(label)),
            Err(e) => {
                warn!("{}", e);
                Ok(HttpResponse::InternalServerError().into())
            }
        })
}

#[post("/Quiz")]
async fn create(state: web::Data<State>, data: web::Json<Quiz>) -> impl Responder {
    state
        .quiz_db
        .send(CreateQuiz {
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

#[put("/Quiz/{id}")]
async fn update(
    state: web::Data<State>,
    data: web::Json<Quiz>,
    uid: web::Path<Uuid>,
) -> impl Responder {
    state
        .quiz_db
        .send(CreateQuiz {
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

#[delete("/Quiz/{id}")]
async fn delete(state: web::Data<State>, uid: web::Path<Uuid>) -> impl Responder {
    let id = uid.into_inner();
    state
        .quiz_db
        .send(DeleteQuiz { id })
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
