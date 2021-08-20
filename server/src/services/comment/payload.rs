use crate::db::comment::models::CommentModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct NewCommentPayload {
    pub post_id: String,

    #[validate(length(min = 1, max = 8000))]
    pub body: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct EditCommentPayload {
    pub comment_id: String,

    #[validate(length(min = 1, max = 8000))]
    pub body: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteCommentPayload {
    pub comment_id: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct GetCommentsByPostPayload {
    pub post_id: String,

    #[validate(range(min = 1))]
    pub page: usize,

    #[validate(range(min = 1))]
    pub per_page: usize,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct GetCommentsByUserPayload {
    pub author_id: String,

    #[validate(range(min = 1))]
    pub page: usize,

    #[validate(range(min = 1))]
    pub per_page: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatedCommentsReturnPayload {
    pub current_page: usize,
    pub max_page: usize,
    pub page: Vec<CommentModel>,
}
