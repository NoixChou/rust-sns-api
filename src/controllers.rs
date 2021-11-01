use actix_web::HttpResponse;
use maplit::hashmap;

use crate::models::error;

pub mod auth_controller;
pub mod user_controller;
pub mod post_controller;

fn invalid_uuid_response() -> HttpResponse {
    HttpResponse::BadRequest().json(
        hashmap! { "error" => error::Error::new(error::ErrorCode::InvalidRequest, "Invalid uuid.") }
    )
}

fn parse_error_response() -> HttpResponse {
    HttpResponse::BadRequest().json(
        hashmap! { "error" => error::Error::new(error::ErrorCode::InvalidRequest, "Failed to parse request.") }
    )
}