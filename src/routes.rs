use crate::controllers::{user_controller, post_controller};
use actix_web::web;

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/users/my", web::get().to(user_controller::show_my))
        .route("/users/{id}", web::get().to(user_controller::show))
        .route("/users", web::post().to(user_controller::create));
}

pub fn posts(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/posts", web::get().to(post_controller::index))
        .route("/posts/{id}", web::get().to(post_controller::show))
        .route("/posts", web::post().to(post_controller::create))
        .route("/posts/{id}", web::delete().to(post_controller::delete))
        .route("/posts/user/{user_id}", web::get().to(post_controller::user_index));
}