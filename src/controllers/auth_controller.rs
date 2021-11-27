use actix_web::{HttpResponse, Responder, web};
use maplit::hashmap;

use crate::controllers::parse_error_response;
use crate::DBConPool;
use crate::models::error::*;
use crate::models::user_credential::*;
use crate::models::user_token::*;
use crate::services::token_authentication::AuthorizedUser;

pub async fn login(credential: Option<web::Json<InputUserCredential>>, db: web::Data<DBConPool>) -> impl Responder {
    let credential = match credential {
        Some(c) => c,
        None => return parse_error_response()
    };
    
    let user = UserCredential::verify_with_input(&credential, &db);
    
    match user {
        Ok(u) => HttpResponse::Ok().json(
            hashmap! {
                "token" => issue_user_token(&u.id, &db)
            }
        ),
        Err(_) => HttpResponse::Unauthorized().json(
            hashmap! {
                "error" => ApiError::new(ApiErrorCode::AuthFailed, "Invalid credentials.")
            }
        ),
    }
}

pub async fn register(new_credential: Option<web::Json<InputUserCredential>>, db: web::Data<DBConPool>) -> impl Responder {
    let new_credential = match new_credential {
        Some(c) => c,
        None => return parse_error_response()
    };
    
    let result = UserCredential::insert(new_credential.0, &db).map_err(
        |e| HttpResponse::BadRequest().json(
            hashmap! {
                "error" => ApiError::new_with_detail(ApiErrorCode::InvalidRequest, "Invalid parameter.", e)
            }
        )
    );
    
    match result {
        Ok(Some(_)) => HttpResponse::Created().finish(),
        Err(e) => e,
        _ => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn show_me(authorized_user: web::ReqData<AuthorizedUser>) -> impl Responder {
    HttpResponse::Ok().json(
        hashmap! {
            "credential" => authorized_user.credential.clone()
        }
    )
}

pub async fn logout(authorized_user: web::ReqData<AuthorizedUser>, db: web::Data<DBConPool>) -> impl Responder {
    UserToken::revoke(&authorized_user.token, &db)
        .map(|_| HttpResponse::NoContent().finish())
        .map_err(
            |e| e.error_response()
        )
}

fn issue_user_token(user_id: &String, db: &DBConPool) -> String {
    UserToken::issue(user_id, &db).expect("Failed to issue token")
}