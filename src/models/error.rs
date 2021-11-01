#[derive(Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "invalid_request")] InvalidRequest,
    #[serde(rename = "not_found")] NotFound,
    #[serde(rename = "not_allowed")] NotAllowed,
    #[serde(rename = "auth_failed")] Auth,
    #[serde(rename = "server_error")] Server,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub code: ErrorCode,
    pub message: String
}

impl Error {
    pub fn new(code: ErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string()
        }
    }

    pub fn new_with_detail<T: serde::Serialize>(code: ErrorCode, message: &str, detail: T) -> ErrorWithDetail<T> {
        ErrorWithDetail::<T> {
            code,
            message: message.to_string(),
            detail,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorWithDetail<T>
where
    T: serde::Serialize
{
    pub code: ErrorCode,
    pub message: String,
    pub detail: T
}

