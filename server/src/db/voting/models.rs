use crate::db::schema::upvotes;
use crate::db::schema::downvotes;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct UpvoteModel {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid
}

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct DownvoteModel {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid
}

#[derive(Insertable)]
#[table_name = "upvotes"]
pub struct NewUpvote {
    pub post_id: Uuid,
    pub user_id: Uuid
}

#[derive(Insertable)]
#[table_name = "downvotes"]
pub struct NewDownvote {
    pub post_id: Uuid,
    pub user_id: Uuid
}