use actix_web::HttpResponse;
use maplit::hashmap;

use crate::models::error;
use crate::services::token_authentication::AuthorizedUser;

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

fn is_created_user(authorized_user: &AuthorizedUser) -> bool {
    authorized_user.user.is_some()
}