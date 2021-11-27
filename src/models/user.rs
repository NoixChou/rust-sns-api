use diesel::prelude::*;
use log::error;
use serde::Serialize;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::DBConPool;
use crate::models::error::{ApiError, ApiErrorCode};
use crate::models::get_now_naive_date_time;
use crate::schema::users;

#[derive(Deserialize, Validate)]
pub struct InputUser {
    #[validate(length(min = 3, max = 20))]
    pub id_name: String,
    #[validate(length(min = 1, max = 100))]
    pub display_name: String,
    #[validate(length(max = 300))]
    pub description: String,
    #[validate(custom = "validate_birthday")]
    pub birthday: Option<chrono::NaiveDate>,
    #[validate(custom = "validate_website", length(max = 100))]
    pub website: String,
    pub is_private: bool,
}

#[derive(AsChangeset, Deserialize, Validate)]
#[table_name = "users"]
pub struct InputPatchUser {
    #[validate(length(min = 1, max = 100))]
    pub display_name: Option<String>,
    #[validate(length(max = 300))]
    pub description: Option<String>,
    #[validate(custom = "validate_birthday")]
    pub birthday: Option<chrono::NaiveDate>,
    #[validate(custom = "validate_website", length(max = 100))]
    pub website: String,
    pub is_private: bool,
}

fn validate_birthday(birthday: &chrono::NaiveDate) -> Result<(), ValidationError> {
    if *birthday > get_now_naive_date_time().date() {
        return Err(ValidationError::new("future_birthday"));
    }
    
    Ok(())
}

fn validate_website(website: &String) -> Result<(), ValidationError> {
    if website.is_empty() {
        return Ok(());
    }
    
    match validator::validate_url(website) {
        true => Ok(()),
        false => Err(ValidationError::new("url"))
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "users"]
pub struct InsertableUser {
    pub id: String,
    pub id_name: String,
    pub display_name: String,
    pub description: String,
    pub birthday: Option<chrono::NaiveDate>,
    pub website: String,
    pub is_private: bool,
}

impl InsertableUser {
    pub fn new(new_user: InputUser, user_id: String) -> Result<Self, ValidationErrors> {
        new_user.validate()?;
        
        Ok(Self {
            id: user_id,
            id_name: new_user.id_name,
            display_name: new_user.display_name,
            description: new_user.description,
            birthday: new_user.birthday,
            website: new_user.website,
            is_private: new_user.is_private,
        })
    }
}

#[derive(Serialize)]
pub struct FilteredUser(User);

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserTagged {
    User(FilteredUser)
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Clone)]
pub struct User {
    pub id: String,
    pub id_name: String,
    pub display_name: String,
    pub description: String,
    pub birthday: Option<chrono::NaiveDate>,
    pub website: String,
    pub is_private: bool,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn wrap_tagged(self) -> UserTagged {
        UserTagged::User(self.filter_for_response())
    }
    
    pub fn filter_for_response(&self) -> FilteredUser {
        let mut user = self.clone();
        if !user.is_private { return FilteredUser(user); }
        
        user.birthday = None;
        
        FilteredUser(user)
    }
    
    pub fn insert(user: InputUser, user_id: String, db: &DBConPool) -> Result<Option<String>, ValidationErrors> {
        use crate::schema::users::dsl;
    
        let insertable_user = InsertableUser::new(user, user_id)?;
    
        let modified_rows_count = diesel::insert_into(dsl::users)
            .values(&insertable_user)
            .execute(&crate::get_db_connection(db));
    
        response_item_insertion_result!(modified_rows_count, insertable_user.id)
    }
    
    pub fn update(user: InputPatchUser, user_id: &String, db: &DBConPool) -> Result<User, ApiError> {
        use crate::schema::users::dsl;
        
        if let Err(_) = user.validate() {
            return Err(ApiError::new(ApiErrorCode::InvalidRequest, "Invalid parameter."));
        }
    
        diesel::update(dsl::users
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::id.eq(user_id))
        )
            .set(&user)
            .execute(&crate::get_db_connection(db))
            .map(|_| {
                Self::fetch_by_id(user_id, db)
                    .unwrap_or_else(|e| panic!("User updated but failed to fetch ({}): {}", user_id, e))
            })
            .map_err(|_| ApiError::new(ApiErrorCode::NotFound, "User does not exist."))
    }
    
    pub fn fetch_by_id(user_id: &String, db: &DBConPool) -> QueryResult<User> {
        use crate::schema::users::dsl;
        
        dsl::users
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::id.eq(user_id))
            .first::<User>(&crate::get_db_connection(db))
    }
}

