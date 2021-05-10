use crate::controllers::{users, posts};
use actix_web::web;

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/users/my", web::get().to(users::show_my))
        .route("/users/{id}", web::get().to(users::show))
        .route("/users", web::post().to(users::create));
}

pub fn posts(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/posts", web::get().to(posts::index))
        .route("/posts/{id}", web::get().to(posts::show))
        .route("/posts", web::post().to(posts::create))
        .route("/posts/{id}", web::delete().to(posts::delete))
        .route("/posts/user/{user_id}", web::get().to(posts::user_index));
}