table! {
    comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        author_id -> Nullable<Uuid>,
        body -> Varchar,
    }
}

table! {
    downvotes (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
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

table! {
    upvotes (id) {
        id -> Uuid,
        post_id -> Uuid,
        user_id -> Uuid,
    }
}

table! {
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

joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(downvotes -> posts (post_id));
joinable!(downvotes -> users (user_id));
joinable!(posts -> users (post_author));
joinable!(upvotes -> posts (post_id));
joinable!(upvotes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    comments,
    downvotes,
    posts,
    upvotes,
    users,
);
