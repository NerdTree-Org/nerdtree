// @generated automatically by Diesel CLI.

diesel::table! {
    comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        author_id -> Nullable<Uuid>,
        body -> Varchar,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    downvotes (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    posts (id) {
        id -> Uuid,
        is_approved -> Bool,
        title -> Varchar,
        thumbnail -> Nullable<Varchar>,
        body -> Text,
        creation_date -> Timestamp,
        approval_date -> Nullable<Timestamp>,
        post_author -> Nullable<Uuid>,
    }
}

diesel::table! {
    upvotes (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        password -> Varchar,
        email -> Varchar,
        profile_pic -> Nullable<Varchar>,
        is_admin -> Bool,
        facebook_id -> Varchar,
        discord_token -> Uuid,
        is_discord_token_used -> Bool,
        joined_date -> Timestamp,
    }
}

diesel::joinable!(comments -> posts (post_id));
diesel::joinable!(comments -> users (author_id));
diesel::joinable!(downvotes -> posts (post_id));
diesel::joinable!(downvotes -> users (user_id));
diesel::joinable!(posts -> users (post_author));
diesel::joinable!(upvotes -> posts (post_id));
diesel::joinable!(upvotes -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    comments,
    downvotes,
    posts,
    upvotes,
    users,
);
