use chrono::TimeZone;
use serde::Serialize;

macro_rules! response_item_insertion_result {
    ($modified_rows_count:expr, $response_data:expr) => {
        match $modified_rows_count {
            Ok(count) if count < 1 => panic!("diesel::insert_into query was successfully executed, but no rows were changed."),
            Ok(count) if count > 1 => panic!("diesel::insert_into query was successfully executed, but more than one rows were changed."),
            Ok(_) => Ok(Some($response_data)),
            Err(e) => {
                error!("query was failed: {:?}", e);
                Ok(None)
            }
        }
    }
}

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

pub(in crate::models) fn serialize_option_naive_dt<S>(date: &Option<chrono::NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    match date {
        None => serializer.serialize_none(),
        Some(d) => chrono::Utc.from_utc_datetime(d).serialize(serializer)
    }
}

pub(in crate::models) fn get_now_date_time() -> chrono::DateTime<chrono::Local> {
    chrono::Local::now()
}

pub(in crate::models) fn get_now_naive_date_time() -> chrono::NaiveDateTime {
    get_now_date_time().naive_local()
}
