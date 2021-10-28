use diesel::prelude::*;
use crate::models::error::*;
use crate::models::user::*;
use crate::schema::users::dsl;
use crate::DBConPool;
use actix_web::{HttpRequest, HttpResponse, Responder, web, ResponseError};
use validator::{Validate, ValidationErrors, ValidationError};
use uuid::Uuid;
use maplit::hashmap;
use std::str::FromStr;
use chrono::TimeZone;
use diesel::{QueryDsl, ExpressionMethods};
use actix_web::body::Body;
use std::any::Any;

fn invalid_uuid_response() -> HttpResponse {
    HttpResponse::BadRequest().json(
        hashmap! { "error" => Error::new(ErrorCode::InvalidRequest, "Invalid uuid.") }
    )
}

fn parse_error_response() -> HttpResponse {
    HttpResponse::BadRequest().json(
        hashmap! { "error" => Error::new(ErrorCode::InvalidRequest, "Failed to parse request.") }
    )
}

pub async fn show(user_id: Option<web::Path<uuid::Uuid>>, db: web::Data<DBConPool>) -> impl Responder {
    use crate::schema::users::dsl::*;

    let user_id= match user_id {
        None => return invalid_uuid_response(),
        Some(u) => u
    };

    let mut user = User::get_by_id(user_id.to_string(), &db.get().unwrap());


    match user {
        Ok(mut user) => HttpResponse::Ok().json(
            hashmap! { "user" => user.filter_for_response() }
        ),
        Err(diesel::NotFound) => HttpResponse::NotFound().json(
            hashmap! { "error" => Error::new(ErrorCode::NotFound, "User does not exist.") }
        ),
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn create(new_user: Option<web::Json<NewUser>>, db: web::Data<DBConPool>) -> impl Responder {
    let new_user = match new_user {
        None => return parse_error_response(),
        Some(u) => u
    };

    let validate_result: Result<_, ValidationErrors> = new_user.validate();

    match validate_result {
        Ok(_) => (),
        Err(e) => {
            return HttpResponse::BadRequest().json(
                hashmap! { "error" => Error::new_with_detail(ErrorCode::InvalidRequest, "Invalid parameter.", e) }
            )
        }
    }

    let insert_user = InsertableUser::new(new_user.0);

    let result = diesel::insert_into(dsl::users)
        .values(&insert_user)
        .execute(&db.get().expect("Failed to establish DB connection"));

    match result {
        Ok(_) => HttpResponse::Ok().json(
            hashmap! { "user" => User::get_by_id(insert_user.id, &db.get().unwrap()).unwrap() }
        ),
        Err(e) => HttpResponse::InternalServerError().finish()
    }
}

pub async fn delete(web::Path(id): web::Path<uuid::Uuid>) -> impl Responder {
    format!("profile delete {}", id.to_string())
}

pub async fn show_my() -> impl Responder {
    "profile my"
}