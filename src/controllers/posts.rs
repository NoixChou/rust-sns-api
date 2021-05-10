use crate::models::post;
use actix_web::{HttpRequest, HttpResponse, Responder};

pub async fn index() -> impl Responder {
    "index"
}

pub async fn show() -> impl Responder {
    "show"
}

pub async fn create() -> impl Responder {
    "create"
}

pub async fn delete() -> impl Responder {
    "delete"
}

pub async fn user_index() -> impl Responder {
    "user_index"
}