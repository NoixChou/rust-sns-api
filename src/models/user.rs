use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: uuid::Uuid,
    pub id_name: String,
    pub display_name: String,
    pub description: String,
    pub birthday: chrono::DateTime<chrono::Local>,
    pub content: String,
    pub is_draft: bool,
}
