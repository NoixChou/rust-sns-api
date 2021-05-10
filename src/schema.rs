table! {
    posts (id) {
        id -> Char,
        message -> Varchar,
        author_id -> Char,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Char,
        id_name -> Varchar,
        display_name -> Varchar,
        description -> Nullable<Varchar>,
        birthday -> Nullable<Date>,
        website -> Nullable<Varchar>,
        is_private -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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

allow_tables_to_appear_in_same_query!(
    posts,
    users,
    user_images,
);
