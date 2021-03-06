use actix_web::web;

use crate::controllers::{auth_controller, post_controller, user_controller};
use crate::services::token_authentication::TokenAuthentication;

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/users")
            .service(web::resource("/me")
                .wrap(TokenAuthentication::required())
                .route(web::get().to(user_controller::show_me))
                .route(web::patch().to(user_controller::update_me))
            )
            .service(web::resource("/me/posts")
                .wrap(TokenAuthentication::required())
                .route(web::get().to(post_controller::my_index))
            )
            .service(web::resource("/{id}")
                .wrap(TokenAuthentication::unnecessary())
                .route(web::get().to(user_controller::show))
            )
            .service(web::resource("")
                .wrap(TokenAuthentication::required())
                .route(web::post().to(user_controller::create))
            )
            .service(web::resource("/{id}/posts")
                .wrap(TokenAuthentication::required())
                .route(web::get().to(post_controller::users_index))
            )
        );
}

pub fn posts(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/posts")
            .wrap(TokenAuthentication::required())
            .service(web::resource("")
                .route(web::get().to(post_controller::index))
                .route(web::post().to(post_controller::create))
            )
            .service(web::resource("/{id}")
                .route(web::get().to(post_controller::show))
                .route(web::delete().to(post_controller::delete))
            )
        );
}

pub fn auth(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::scope("/auth")
            .route("/login", web::post().to(auth_controller::login))
            .route("/register", web::post().to(auth_controller::register))
            .service(web::resource("/logout")
                .wrap(TokenAuthentication::required())
                .route(web::delete().to(auth_controller::logout))
            )
            .service(web::resource("/me")
                .wrap(TokenAuthentication::required())
                .route(web::get().to(auth_controller::show_me))
            )
        );
}