use actix_web::{HttpResponse, Responder, web};
use actix_web::web::Data;
use maplit::hashmap;

use crate::controllers::{invalid_uuid_response, parse_error_response};
use crate::DBConPool;
use crate::models::error::{ApiError, ApiErrorCode};
use crate::models::post::{InputPost, Post};
use crate::models::user::User;
use crate::services::token_authentication::AuthorizedUser;

pub async fn index() -> impl Responder {
    "index"
}

pub async fn show(post_id: Option<web::Path<uuid::Uuid>>, db: web::Data<DBConPool>) -> impl Responder {
    let post_id = match post_id {
        None => return invalid_uuid_response(),
        Some(u) => u
    };
    
    let post = Post::fetch_by_id(&post_id.to_string(), &db);
    
    match post {
        Ok(p) => {
            let u = User::fetch_by_id(&p.author_id, &db).expect("User does not exists but `id` found in Post.");
            HttpResponse::Ok().json(p.with_user(u.filter_for_response()))
        }
        Err(diesel::NotFound) => HttpResponse::NotFound().json(
            hashmap! { "error" => ApiError::new(ApiErrorCode::NotFound, "Post does not exist.") }
        ),
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn create(new_post: Option<web::Json<InputPost>>, authorized_user: web::ReqData<AuthorizedUser>, db: web::Data<DBConPool>) -> impl Responder {
    let user = match &authorized_user.user {
        Some(u) => u,
        None => return ApiError::new(ApiErrorCode::NotFound, "Create user first.").error_response()
    };
    
    let new_post = match new_post {
        None => return parse_error_response(),
        Some(p) => p
    };
    
    let result = Post::insert(new_post.0, user, &db).map_err(
        |e| HttpResponse::BadRequest().json(
            hashmap! { "error" => ApiError::new_with_detail(ApiErrorCode::InvalidRequest, "Invalid parameter.", e) }
        )
    );
    
    match result {
        Ok(Some(id)) => {
            let created_post = Post::fetch_by_id(&id, &db).unwrap_or_else(|_| panic!("Failed to create Post {}", id));
            HttpResponse::Ok().json(created_post.wrap_tagged())
        }
        Err(e) => e,
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn delete() -> impl Responder {
    "delete"
}

pub async fn users_index(user_id: Option<web::Path<uuid::Uuid>>, db: web::Data<DBConPool>) -> impl Responder {
    let user_id = match user_id {
        None => return invalid_uuid_response(),
        Some(u) => u
    };
    
    let user = match User::fetch_by_id(&user_id.to_string(), &db) {
        Ok(u) => u,
        Err(_) => return ApiError::new(ApiErrorCode::NotFound, "User not found").error_response()
    };
    
    response_fetch_posts_by_user(&db, &user)
}

pub async fn my_index(authorized_user: web::ReqData<AuthorizedUser>, db: web::Data<DBConPool>) -> impl Responder {
    let user = match &authorized_user.user {
        Some(u) => u,
        None => return ApiError::new(ApiErrorCode::NotFound, "Create user first.").error_response()
    };
    
    response_fetch_posts_by_user(&db, &user)
}

fn response_fetch_posts_by_user(db: &Data<DBConPool>, user: &User) -> HttpResponse {
    match Post::fetch_list_by_author(user, &db) {
        Ok(posts) => {
            HttpResponse::Ok().json(
                posts.wrap_tagged()
            )
        }
        Err(_) => ApiError::new(ApiErrorCode::NotFound, "Posts not found").error_response()
    }
}
