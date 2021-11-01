use actix_web::web;
use diesel::prelude::*;
use serde::Serialize;
use validator::{Validate, ValidationError, ValidationErrors};

use crate::DBConPool;
use crate::schema::users;

#[derive(Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(max = 20))]
    pub id_name: String,
    #[validate(length(max = 100))]
    pub display_name: String,
    #[validate(length(max = 300))]
    pub description: String,
    #[validate(custom = "validate_birthday")]
    pub birthday: Option<chrono::NaiveDate>,
    #[validate(custom = "validate_website", length(max = 100))]
    pub website: String,
    pub is_private: bool,
}

fn validate_birthday(birthday: &chrono::NaiveDate) -> Result<(), ValidationError> {
    if *birthday > chrono::Local::now().date().naive_local() {
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
    pub fn new(new_user: NewUser) -> Result<Self, ValidationErrors> {
        new_user.validate()?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            id_name: new_user.id_name,
            display_name: new_user.display_name,
            description: new_user.description,
            birthday: new_user.birthday,
            website: new_user.website,
            is_private: new_user.is_private,
        })
    }
}

#[derive(Serialize, Deserialize, Queryable)]
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
    pub fn filter_for_response(&mut self) -> &Self {
        if !self.is_private { return self; }
        
        self.birthday = None;
        
        self
    }
    
    pub fn insert(user: NewUser, db: &web::Data<DBConPool>) -> Result<Option<String>, ValidationErrors> {
        use crate::schema::users::dsl;
        
        let insertable_user = InsertableUser::new(user)?;
        
        let modified_rows_count = diesel::insert_into(dsl::users)
            .values(&insertable_user)
            .execute(&db.get().expect("Failed to establish DB connection"));
        
        match modified_rows_count {
            Ok(count) => {
                if count < 1 { return Ok(None); }
                Ok(Some(insertable_user.id))
            }
            _ => Ok(None)
        }
    }
    
    pub fn fetch_by_id(user_id: String, db: &diesel::MysqlConnection) -> QueryResult<User> {
        use crate::schema::users::{dsl, dsl::*};
        dsl::users
            .filter(id.eq(user_id))
            .first::<User>(db)
    }
}


