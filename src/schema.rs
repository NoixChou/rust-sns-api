table! {
    posts (id) {
        id -> Char,
        message -> Varchar,
        author_id -> Char,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Char,
        id_name -> Varchar,
        display_name -> Varchar,
        description -> Varchar,
        birthday -> Nullable<Date>,
        website -> Varchar,
        is_private -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user_credentials (id) {
        id -> Char,
        password_hash -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user_images (id) {
        id -> Char,
        user_id -> Nullable<Char>,
        object_key -> Varchar,
        image_type -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(posts -> users (author_id));
joinable!(users -> user_credentials (id));

allow_tables_to_appear_in_same_query!(
    posts,
    users,
    user_credentials,
    user_images,
);
