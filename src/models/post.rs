use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
    pub id: uuid::Uuid,
    pub content: String,
    pub is_draft: bool,
}
