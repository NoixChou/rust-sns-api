use actix_web::{HttpResponse, Responder, web};
use maplit::hashmap;

use crate::controllers::{invalid_uuid_response, parse_error_response};
use crate::DBConPool;
use crate::models::error::*;
use crate::models::user::*;
use crate::services::token_authentication::AuthorizedUser;

pub async fn show(user_id: Option<web::Path<uuid::Uuid>>, db: web::Data<DBConPool>) -> impl Responder {
    let user_id = match user_id {
        None => return invalid_uuid_response(),
        Some(u) => u
    };
    
    let user = User::fetch_by_id(user_id.to_string(), &db);
    
    match user {
        Ok(user) => HttpResponse::Ok().json(
            hashmap! { "user" => user.filter_for_response() }
        ),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(
            hashmap! { "error" => ApiError::new(ApiErrorCode::NotFound, "User does not exist.") }
        ),
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn create(new_user: Option<web::Json<InputUser>>, authorized_user: Option<web::ReqData<AuthorizedUser>>, db: web::Data<DBConPool>) -> impl Responder {
    let authorized_user = match authorized_user {
        None => return ApiError::new(ApiErrorCode::AuthFailed, "Authorization required.").error_response(),
        Some(u) => {
            if let Some(_) = u.user {
                return ApiError::new(ApiErrorCode::InvalidRequest, "User already created.").error_response();
            };
            u
        }
    };
    
    let new_user = match new_user {
        None => return parse_error_response(),
        Some(u) => u
    };
    
    let result = User::insert(new_user.0, authorized_user.credential.id.clone(), &db).map_err(
        |e| HttpResponse::BadRequest().json(
            hashmap! { "error" => ApiError::new_with_detail(ApiErrorCode::InvalidRequest, "Invalid parameter.", e) }
        )
    );
    
    match result {
        Ok(Some(id)) => {
            let created_user = User::fetch_by_id(id, &db).unwrap_or_else(|_| panic!("Failed to create User {}", authorized_user.credential.id));
            HttpResponse::Ok().json(
                hashmap! { "user" => created_user.filter_for_response()
            })
        },
        Err(e) => e,
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn show_me(authorized_user: web::ReqData<AuthorizedUser>) -> impl Responder {
    authorized_user.user.as_ref()
        .ok_or(ApiError::new(ApiErrorCode::NotFound, "Create user first.").error_response())
        .map(|u| {
            HttpResponse::Ok().json(
                hashmap! { "user" => u.filter_for_response() }
            )
        })
}