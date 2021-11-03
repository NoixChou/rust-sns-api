use chrono::TimeZone;
use serde::Serialize;

pub mod error;
pub mod user;
pub mod user_credential;
pub mod user_token;
pub mod post;

pub(in crate::models) fn serialize_naive_dt<S>(date: &chrono::NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    chrono::Utc.from_utc_datetime(date).serialize(serializer)
}
