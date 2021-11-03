use actix_web::{HttpResponse, Responder, web};
use maplit::hashmap;

use crate::controllers::{invalid_uuid_response, parse_error_response};
use crate::DBConPool;
use crate::models::error::*;
use crate::models::user::*;

pub async fn show(user_id: Option<web::Path<uuid::Uuid>>, db: web::Data<DBConPool>) -> impl Responder {
    let user_id = match user_id {
        None => return invalid_uuid_response(),
        Some(u) => u
    };
    
    let user = User::fetch_by_id(user_id.to_string(), &db);
    
    match user {
        Ok(mut user) => HttpResponse::Ok().json(
            hashmap! { "user" => user.filter_for_response() }
        ),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(
            hashmap! { "error" => ApiError::new(ApiErrorCode::NotFound, "User does not exist.") }
        ),
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn create(new_user: Option<web::Json<InputUser>>, db: web::Data<DBConPool>) -> impl Responder {
    let new_user = match new_user {
        None => return parse_error_response(),
        Some(u) => u
    };
    
    let result = User::insert(new_user.0, &db).map_err(
        |e| HttpResponse::BadRequest().json(
            hashmap! { "error" => ApiError::new_with_detail(ApiErrorCode::InvalidRequest, "Invalid parameter.", e) }
        )
    );
    
    match result {
        Ok(Some(id)) => HttpResponse::Ok().json(
            hashmap! { "user" => User::fetch_by_id(id, &db).unwrap() }
        ),
        Err(e) => e,
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn delete(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    format!("profile delete {}", id.to_string())
}

pub async fn show_my() -> impl Responder {
    "profile my"
}