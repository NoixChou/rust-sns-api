use actix_web::{HttpResponse, Responder, web};
use maplit::hashmap;

use crate::controllers::{invalid_uuid_response, parse_error_response};
use crate::DBConPool;
use crate::models::error::{ApiError, ApiErrorCode};
use crate::models::post::{InputPost, Post};
use crate::models::user::User;
use crate::services::token_authentication::AuthorizedUser;

#[derive(Deserialize)]
pub struct PostIdPagination {
    latest_post_id: Option<uuid::Uuid>,
    oldest_post_id: Option<uuid::Uuid>,
}

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
            HttpResponse::Created().json(
                hashmap! {
                    "post" => hashmap! {
                        "id" => id
                    }
                }
            )
        }
        Err(e) => e,
        _ => HttpResponse::InternalServerError().finish()
    }
}

pub async fn delete() -> impl Responder {
    "delete"
}

pub async fn users_index(user_id: Option<web::Path<uuid::Uuid>>, pagination: web::Query<PostIdPagination>, db: web::Data<DBConPool>) -> impl Responder {
    let user_id = match user_id {
        None => return invalid_uuid_response(),
        Some(u) => u
    };
    
    let user = match User::fetch_by_id(&user_id.to_string(), &db) {
        Ok(u) => u,
        Err(_) => return ApiError::new(ApiErrorCode::NotFound, "User not found").error_response()
    };
    
    response_fetch_posts_by_user(&db, &user, pagination)
}

pub async fn my_index(authorized_user: web::ReqData<AuthorizedUser>, pagination: web::Query<PostIdPagination>, db: web::Data<DBConPool>) -> impl Responder {
    let user = match &authorized_user.user {
        Some(u) => u,
        None => return ApiError::new(ApiErrorCode::NotFound, "Create user first.").error_response()
    };
    
    response_fetch_posts_by_user(&db, &user, pagination)
}

fn response_fetch_posts_by_user(db: &web::Data<DBConPool>, user: &User, pagination: web::Query<PostIdPagination>) -> HttpResponse {
    let fetch_post_fn = |id: uuid::Uuid| {
        Post::fetch_by_id(&id.to_string(), db).ok()
    };
    
    let latest_post = pagination.latest_post_id.and_then(fetch_post_fn);
    let oldest_post = pagination.oldest_post_id.and_then(fetch_post_fn);
    
    match Post::fetch_list_by_author(user, &latest_post, &oldest_post, &db) {
        Ok(posts) => {
            HttpResponse::Ok().json(
                posts.wrap_tagged()
            )
        }
        Err(_) => ApiError::new(ApiErrorCode::NotFound, "Posts not found").error_response()
    }
}
