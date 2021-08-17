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

    pub post_id: String
}