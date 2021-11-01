use actix_web::web;
use argon2::{Algorithm, Argon2, Params, password_hash::rand_core::OsRng, PasswordHasher, Version};
use argon2::password_hash::SaltString;
use diesel::prelude::*;
use validator::{Validate, ValidationErrors};

use crate::DBConPool;
use crate::schema::user_credentials;

#[derive(Deserialize, Validate)]
pub struct NewUserCredential {
    #[validate(length(min = 8))]
    raw_password: String,
    #[validate(email)]
    email: String,
}

#[derive(Insertable)]
#[table_name = "user_credentials"]
pub struct InsertableUserCredential {
    id: String,
    password_hash: String,
    email: String,
}

impl InsertableUserCredential {
    pub fn new(new_credential: NewUserCredential) -> Result<Self, ValidationErrors> {
        new_credential.validate()?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            password_hash: hash_password(&new_credential.raw_password),
            email: new_credential.email,
        })
    }
}

#[derive(Serialize, Queryable)]
pub struct UserCredential {
    id: String,
    #[serde(skip)]
    password: String,
    email: String,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    updated_at: chrono::NaiveDateTime,
    #[serde(skip)]
    deleted_at: Option<chrono::NaiveDateTime>,
}

impl UserCredential {
    pub fn insert(user: NewUserCredential, db: &web::Data<DBConPool>) -> Result<Option<String>, ValidationErrors> {
        use crate::schema::user_credentials::dsl;
        
        let insertable_credential = InsertableUserCredential::new(user)?;
        
        let modified_rows_count = diesel::insert_into(dsl::user_credentials)
            .values(&insertable_credential)
            .execute(&db.get().expect("Failed to establish DB connection"));
        
        match modified_rows_count {
            Ok(count) => {
                if count < 1 { return Ok(None); }
                Ok(Some(insertable_credential.id))
            }
            _ => Ok(None)
        }
    }
    
    pub fn fetch_by_id(user_id: String, db: &MysqlConnection) -> QueryResult<Self> {
        use crate::schema::user_credentials::{dsl, dsl::*};
        dsl::user_credentials
            .filter(deleted_at.is_null())
            .filter(id.eq(user_id))
            .first::<Self>(db)
    }
}

fn hash_password(password: &String) -> String {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(37888, 1, 1, None).expect("Failed to initialize argon2"),
    )
        .hash_password(
            password.as_bytes(),
            &SaltString::generate(&mut OsRng),
        ).expect("Failed to argon2 hashing").to_string()
}