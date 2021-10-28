use diesel::prelude::*;
use validator::{Validate, ValidationError};
use crate::schema::*;
use crate::schema::users::dsl;
use chrono::{NaiveDate, TimeZone};
use serde::Serialize;

#[derive(Serialize, Deserialize, Validate)]
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
        return Ok(())
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
    pub fn new(new_user: NewUser) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            id_name: new_user.id_name,
            display_name: new_user.display_name,
            description: new_user.description,
            birthday: new_user.birthday,
            website: new_user.website,
            is_private: new_user.is_private
        }
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
    #[serde(serialize_with = "serialize_naive_dt")]
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "serialize_naive_dt")]
    pub updated_at: chrono::NaiveDateTime,
    #[serde(skip)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl User {
    pub fn filter_for_response(&mut self) -> &Self {
        if !self.is_private { return self }

        self.birthday = None;

        self
    }

    pub fn get_by_id(user_id: String, db: &diesel::MysqlConnection) -> QueryResult<User> {
        use crate::schema::users::dsl::*;

        dsl::users
            .filter(id.eq(user_id))
            .first::<User>(db)
    }
}

pub fn serialize_naive_dt<S>(date: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    chrono::Local.from_utc_datetime(date).serialize(serializer)
}
