use crate::db::schema::downvotes;
use crate::db::schema::upvotes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct UpvoteModel {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Queryable, Clone, Serialize, Deserialize)]
pub struct DownvoteModel {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = upvotes)]
pub struct NewUpvote {
    pub post_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = downvotes)]
pub struct NewDownvote {
    pub post_id: Uuid,
    pub user_id: Uuid,
}
