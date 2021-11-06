use actix_web::http::header;
use actix_web::HttpResponse;
use maplit::hashmap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    InvalidRequest,
    NotFound,
    NotAllowed,
    AuthFailed,
    InvalidToken,
    ServerError,
}

#[derive(Serialize, Deserialize)]
pub struct ApiError {
    pub code: ApiErrorCode,
    pub message: String,
}

impl ApiError {
    pub fn new(code: ApiErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
    
    pub fn new_with_detail<T: serde::Serialize>(code: ApiErrorCode, message: &str, detail: T) -> ApiErrorWithDetail<T> {
        ApiErrorWithDetail::<T> {
            code,
            message: message.to_string(),
            detail,
        }
    }
    
    pub fn error_response(&self) -> HttpResponse {
        match self.code {
            ApiErrorCode::InvalidRequest => HttpResponse::BadRequest(),
            ApiErrorCode::NotFound => HttpResponse::NotFound(),
            ApiErrorCode::NotAllowed => HttpResponse::MethodNotAllowed(),
            ApiErrorCode::AuthFailed => HttpResponse::Unauthorized().header(header::WWW_AUTHENTICATE, "Bearer").take(),
            ApiErrorCode::InvalidToken => HttpResponse::Unauthorized().header(header::WWW_AUTHENTICATE, "Bearer error=\"invalid_token\"").take(),
            ApiErrorCode::ServerError => HttpResponse::InternalServerError(),
        }.json(
            hashmap! { "error" => self }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiErrorWithDetail<T>
    where
        T: serde::Serialize
{
    pub code: ApiErrorCode,
    pub message: String,
    pub detail: T,
}

