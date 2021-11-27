use diesel::prelude::*;
use log::error;
use serde::Serialize;
use validator::{Validate, ValidationErrors};

use crate::DBConPool;
use crate::models::get_now_naive_date_time;
use crate::models::user::{FilteredUser, User};
use crate::schema::posts;

#[derive(Deserialize, Validate)]
pub struct InputPost {
    #[validate(length(min = 1, max = 1000))]
    pub content: String,
    pub is_publish: bool,
}

#[derive(Insertable, Deserialize)]
#[table_name = "posts"]
pub struct InsertablePost {
    pub id: String,
    pub content: String,
    pub author_id: String,
    pub published_at: Option<chrono::NaiveDateTime>,
}

impl InsertablePost {
    pub fn new(new_post: InputPost, author: &User) -> Result<Self, ValidationErrors> {
        new_post.validate()?;
        
        Ok(Self {
            id: uuid::Uuid::new_v4().to_string(),
            content: new_post.content,
            author_id: author.id.clone(),
            published_at: new_post.is_publish.then(|| get_now_naive_date_time()),
        })
    }
}

#[derive(Serialize)]
pub struct Posts(Vec<PostWithUser>);

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTagged {
    Post(Post),
    Posts(Posts),
}

#[derive(Serialize)]
pub struct PostWithUser {
    user: FilteredUser,
    post: Post,
}

#[derive(Serialize, Deserialize, Identifiable, Queryable, Associations, Clone)]
#[belongs_to(User, foreign_key = "author_id")]
pub struct Post {
    pub id: String,
    pub content: String,
    pub author_id: String,
    #[serde(skip_serializing)]
    pub created_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_naive_dt")]
    pub updated_at: chrono::NaiveDateTime,
    #[serde(serialize_with = "crate::models::serialize_option_naive_dt")]
    pub published_at: Option<chrono::NaiveDateTime>,
    #[serde(skip)]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl Posts {
    fn new(posts: Vec<PostWithUser>) -> Self {
        Self(posts)
    }
    
    pub fn wrap_tagged(self) -> PostTagged {
        PostTagged::Posts(self)
    }
}

impl Post {
    pub fn wrap_tagged(self) -> PostTagged {
        PostTagged::Post(self)
    }
    
    pub fn with_user(self, user: FilteredUser) -> PostWithUser {
        PostWithUser { user, post: self }
    }
    
    pub fn insert(post: InputPost, author_user: &User, db: &DBConPool) -> Result<Option<String>, ValidationErrors> {
        use crate::schema::posts::dsl;
    
        let insertable_post = InsertablePost::new(post, author_user)?;
    
        let modified_rows_count = diesel::insert_into(dsl::posts)
            .values(&insertable_post)
            .execute(&crate::get_db_connection(db));
    
        response_item_insertion_result!(modified_rows_count, insertable_post.id)
    }
    
    pub fn fetch_by_id(post_id: &String, db: &DBConPool) -> QueryResult<Post> {
        use crate::schema::posts::dsl;
        
        dsl::posts
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::published_at.lt(get_now_naive_date_time()))
            .filter(dsl::id.eq(post_id))
            .first::<Post>(&crate::get_db_connection(db))
    }
    
    pub fn fetch_list(_requested_user: &User, db: &DBConPool) -> QueryResult<Vec<Post>> {
        use crate::schema::posts::dsl;
        
        dsl::posts
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::published_at.lt(get_now_naive_date_time()))
            .order(dsl::published_at.desc())
            .load::<Post>(&crate::get_db_connection(db))
    }
    
    pub fn fetch_list_by_author(author: &User, db: &DBConPool) -> QueryResult<Posts> {
        use crate::schema::posts::dsl;
        
        Post::belonging_to(author)
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::published_at.is_not_null().and(dsl::published_at.lt(get_now_naive_date_time())))
            .order(dsl::published_at.desc())
            .load::<Post>(&crate::get_db_connection(db))
            .map(|posts: Vec<Post>| {
                Posts::new(
                    posts.iter().map(|p| p.clone().with_user(author.filter_for_response())).collect::<Vec<PostWithUser>>()
                )
            })
    }
    
    pub fn fetch_draft_list_by_author(author: &User, db: &DBConPool) -> QueryResult<Vec<Post>> {
        use crate::schema::posts::dsl;
        
        Post::belonging_to(author)
            .filter(dsl::deleted_at.is_null())
            .filter(dsl::published_at.is_null().or(dsl::published_at.gt(get_now_naive_date_time())))
            .order(dsl::published_at.desc())
            .load::<Post>(&crate::get_db_connection(db))
    }
}