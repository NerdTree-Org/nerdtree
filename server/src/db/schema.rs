table! {
    comments (id) {
        id -> Uuid,
        post_id -> Uuid,
        author_id -> Uuid,
        body -> Varchar,
        upvotes -> Int4,
    }
}

table! {
    posts (id) {
        id -> Uuid,
        is_approved -> Bool,
        title -> Varchar,
        thumbnail -> Varchar,
        body -> Text,
        creation_date -> Timestamp,
        approval_date -> Nullable<Timestamp>,
        post_author -> Nullable<Uuid>,
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
    }
}

joinable!(comments -> posts (post_id));
joinable!(comments -> users (author_id));
joinable!(posts -> users (post_author));

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
    users,
);
