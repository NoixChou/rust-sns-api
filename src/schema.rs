table! {
    posts (id) {
        id -> Bpchar,
        content -> Varchar,
        author_id -> Bpchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        published_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user_credentials (id) {
        id -> Bpchar,
        password_hash -> Varchar,
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user_images (id) {
        id -> Bpchar,
        user_id -> Nullable<Bpchar>,
        object_key -> Varchar,
        image_type -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    user_tokens (token) {
        token -> Bpchar,
        user_id -> Bpchar,
        expired_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Bpchar,
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

joinable!(posts -> users (author_id));
joinable!(user_images -> users (user_id));
joinable!(user_tokens -> user_credentials (user_id));
joinable!(users -> user_credentials (id));

allow_tables_to_appear_in_same_query!(
    posts,
    user_credentials,
    user_images,
    user_tokens,
    users,
);
