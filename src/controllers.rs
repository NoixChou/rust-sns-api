use actix_web::HttpResponse;
use maplit::hashmap;

use crate::models::error;

pub mod auth_controller;
pub mod user_controller;
pub mod post_controller;

fn invalid_uuid_response() -> HttpResponse {
    HttpResponse::BadRequest().json(
        hashmap! { "error" => error::ApiError::new(error::ApiErrorCode::InvalidRequest, "Invalid uuid.") }
    )
}

fn parse_error_response() -> HttpResponse {
    HttpResponse::BadRequest().json(
        hashmap! { "error" => error::ApiError::new(error::ApiErrorCode::InvalidRequest, "Failed to parse request.") }
    )
}