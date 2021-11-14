use std::{
    future::{Future, ready, Ready},
    pin::Pin,
    task::{Context, Poll},
};

use actix_web::{Error, HttpMessage, web};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header::Header;
use actix_web::web::Data;
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

use crate::DBConPool;
use crate::models::error::{ApiError, ApiErrorCode};
use crate::models::user::User;
use crate::models::user_credential::UserCredential;
use crate::models::user_token::UserToken;

pub struct TokenAuthentication {
    auth_required: bool,
}

impl TokenAuthentication {
    pub fn required() -> Self {
        Self { auth_required: true }
    }
    
    pub fn unnecessary() -> Self {
        Self { auth_required: false }
    }
}

impl<S, B> Transform<S> for TokenAuthentication
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = TokenAuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(TokenAuthenticationMiddleware {
            service,
            auth_required: self.auth_required,
        }))
    }
}

pub struct TokenAuthenticationMiddleware<S> {
    service: S,
    auth_required: bool,
}

impl<S, B> Service for TokenAuthenticationMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }
    
    fn call(&mut self, request: ServiceRequest) -> Self::Future {
        let database = request.app_data::<web::Data<DBConPool>>().expect("DBConPool was not wrapped in App");
        
        let result = match Authorization::<Bearer>::parse(&request) {
            Ok(t) => {
                validate_token(&request, &database, t.into_scheme().token().to_string())
            }
            Err(_) if !self.auth_required => {
                Ok(())
            }
            Err(_) => {
                Err(ApiError::new(ApiErrorCode::AuthFailed, "Authorization required."))
            }
        };
        
        match result {
            Ok(_) => Box::pin(self.service.call(request)),
            Err(e) => Box::pin(ready(Err(Error::from(e.error_response()))))
        }
    }
}

#[derive(Clone)]
pub struct AuthorizedUser {
    pub token: String,
    pub credential: UserCredential,
    pub user: Option<User>,
}

fn validate_token(request: &ServiceRequest, database: &Data<DBConPool>, t: String) -> Result<(), ApiError> {
    UserToken::verify_token(&t, &database)
        .map(|t| {
            request.extensions_mut().insert(
                AuthorizedUser {
                    token: t.token,
                    credential: UserCredential::fetch_by_id(&t.user_id, &database).expect("Token was authenticated but failed to fetch UserCredential"),
                    user: User::fetch_by_id(&t.user_id, &database).ok(),
                }
            );
        })
}