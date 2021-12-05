use crate::db::post::models::PostModel;
use awmpde::{File, FromActixMultipart};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct NewPostPayload {
    #[validate(length(min = 3, max = 255))]
    pub title: String,

    pub body: String,
}

#[derive(FromActixMultipart)]
pub struct UploadThumbnailForm {
    pub post_id: String,

    pub thumbnail: File<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBodyPayload {
    pub body: String,
    pub post_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApprovePostPayload {
    pub post_id: String,
    pub approval_state: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DeletePostPayload {
    pub post_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusPayload {
    pub success: bool,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateTitlePayload {
    #[validate(length(min = 3, max = 255))]
    pub new_title: String,

    pub post_id: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PaginatePostsPayload {
    #[validate(range(min = 1))]
    pub page: usize,

    #[validate(range(min = 1))]
    pub per_page: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PaginatePostsReturnPayload {
    pub current_page: usize,
    pub max_page: usize,
    pub page: Vec<PostModel>,
}

#[derive(Serialize, Deserialize)]
pub struct GetPostByIdPayload {
    pub post_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetPostsByAuthorId {
    pub author_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddUpvotePayload {
    pub post_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetVotesPayload {
    pub post_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetVotesReturnPayload {
    pub votes: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserVoteForPostPayload {
    pub post_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetUserVoteForPostReturnPayload {
    /// -1 when downvote, 0 when no vote, 1 when there's upvote
    pub vote: i8,
}
