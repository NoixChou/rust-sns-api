use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::{rand_core::OsRng, SaltString};
use diesel::prelude::*;
use log::error;
use rand::Rng;
use validator::{Validate, ValidationErrors};

use crate::DBConPool;
use crate::schema::user_credentials;

#[derive(Deserialize, Validate)]
pub struct InputUserCredential {
    #[validate(length(min = 8))]
    #[serde(rename = "password")]
    pub raw_password: String,
    #[validate(email)]
    pub email: String,
}

#[derive(Insertable)]
#[table_name = "user_credentials"]
pub struct InsertableUserCredential {
    id: String,
    password_hash: String,
    email: String,
}

impl InsertableUserCredential {
    pub fn new(new_credential: InputUserCredential) -> Result<Self, ValidationErrors> {
        new_credential.validate()?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            password_hash: hash_password(&new_credential.raw_password),
            email: new_credential.email,
        })
    }
}

#[derive(Serialize, Queryable, Clone)]
pub struct UserCredential {
    pub id: String,
    #[serde(skip)]
    pub password_hash: String,
    pub email: String,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl UserCredential {
    pub fn insert(user: InputUserCredential, db: &DBConPool) -> Result<Option<String>, ValidationErrors> {
        use crate::schema::user_credentials::dsl;
    
        let insertable_credential = InsertableUserCredential::new(user)?;
    
        let modified_rows_count = diesel::insert_into(dsl::user_credentials)
            .values(&insertable_credential)
            .execute(&crate::get_db_connection(db));
    
        response_item_insertion_result!(modified_rows_count, insertable_credential.id)
    }
    
    pub fn fetch_by_id(user_id: &String, db: &DBConPool) -> QueryResult<Self> {
        use crate::schema::user_credentials::dsl;
        
        dsl::user_credentials
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::id.eq(user_id))
            .first::<Self>(&crate::get_db_connection(db))
    }
    
    pub fn fetch_by_email(email: &String, db: &DBConPool) -> QueryResult<Self> {
        use crate::schema::user_credentials::dsl;
        
        dsl::user_credentials
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::email.eq(email))
            .first::<Self>(&crate::get_db_connection(db))
    }
    
    pub fn verify_with_input(input_credential: &InputUserCredential, db: &DBConPool) -> Result<Self, ()> {
        let stored_credential = Self::fetch_by_email(&input_credential.email, db)
            .map(|c| {
                build_argon2().verify_password(
                    input_credential.raw_password.as_bytes(),
                    &PasswordHash::new(&c.password_hash)?,
                ).map(|_| c)
            });
        
        match stored_credential {
            Ok(Ok(c)) => Ok(c),
            Ok(Err(_)) => Err(()),
            _ => {
                std::thread::sleep(rand::thread_rng().gen_range(std::time::Duration::from_millis(5)..std::time::Duration::from_millis(50)));
                Err(())
            }
        }
    }
}

fn build_argon2() -> Argon2<'static> {
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(37888, 1, 1, None).expect("Failed to initialize argon2"),
    )
}

fn hash_password(password: &String) -> String {
    build_argon2().hash_password(
        password.as_bytes(),
        &SaltString::generate(&mut OsRng),
    ).expect("Failed to argon2 hashing").to_string()
}
