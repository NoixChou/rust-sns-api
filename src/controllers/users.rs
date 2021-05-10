use crate::models::user;
use actix_web::{HttpRequest, HttpResponse, Responder};
use uuid::Uuid;
use maplit::hashmap;

pub async fn show() -> impl Responder {
    "profile show"
}

pub async fn create() -> impl Responder {
    "profile create"
}

pub async fn delete() -> impl Responder {
    "profile delete"
}

pub async fn show_my() -> impl Responder {
    "profile my"
}