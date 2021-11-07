use diesel::prelude::*;

use crate::DBConPool;
use crate::models::error::{ApiError, ApiErrorCode};
use crate::schema::user_tokens;

struct NewUserToken {
    user_id: String,
    validity_duration: chrono::Duration,
}

#[derive(Insertable)]
#[table_name = "user_tokens"]
pub struct InsertableUserToken {
    token: String,
    user_id: String,
    expired_at: chrono::NaiveDateTime,
    created_at: chrono::NaiveDateTime,
}

impl InsertableUserToken {
    fn new(new_token: &NewUserToken) -> Self {
        let now_datetime = chrono::Local::now();
        
        Self {
            token: uuid::Uuid::new_v4().to_string(),
            user_id: new_token.user_id.clone(),
            expired_at: (now_datetime + new_token.validity_duration).naive_local(),
            created_at: now_datetime.naive_local(),
        }
    }
}

#[derive(Serialize, Queryable)]
pub struct UserToken {
    pub token: String,
    #[serde(skip)]
    pub user_id: String,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub expired_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

macro_rules! filter_for_get_by_token {
    ($token:expr, $query:expr) => {
        $query
            .filter(dsl::expired_at.gt(chrono::Local::now().naive_local()))
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::token.eq($token))
    }
}

impl UserToken {
    pub fn issue(user_id: &String, db: &DBConPool) -> Option<String> {
        use crate::schema::user_tokens::dsl;
        
        let user = NewUserToken { user_id: user_id.clone(), validity_duration: chrono::Duration::days(30) };
        
        let insertable_token = InsertableUserToken::new(&user);
        
        let modified_rows_count = diesel::insert_into(dsl::user_tokens)
            .values(&insertable_token)
            .execute(&crate::get_db_connection(db));
        
        match modified_rows_count {
            Ok(count) if count > 0 => {
                Some(insertable_token.token)
            }
            _ => None
        }
    }
    
    pub fn revoke(token: &String, db: &DBConPool) -> Result<(), ApiError> {
        use crate::schema::user_tokens::dsl;
        
        filter_for_get_by_token!(token, diesel::delete(dsl::user_tokens))
            .execute(&crate::get_db_connection(db))
            .map(|_| ())
            .map_err(|_| ApiError::new(ApiErrorCode::NotFound, "Token not found."))
    }
    
    pub fn verify_token(token: &String, db: &DBConPool) -> Result<Self, ApiError> {
        use crate::schema::user_tokens::dsl;
        
        let token: QueryResult<Self> = filter_for_get_by_token!(token, dsl::user_tokens)
            .first::<Self>(&crate::get_db_connection(db));
        
        token
            .map_err(|_| ApiError::new(ApiErrorCode::InvalidToken, "Invalid token."))
    }
}