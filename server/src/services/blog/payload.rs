use serde::{Deserialize, Serialize};
use validator::Validate;
use awmpde::{FromActixMultipart, File};

#[derive(Serialize, Deserialize, Validate)]
pub struct NewPostPayload {
    #[validate(length(min = 3, max = 255))]
    pub title: String,

    pub body: String
}

#[derive(FromActixMultipart)]
pub struct UploadThumbnailForm {
    pub post_id: String,

    pub thumbnail: File<Vec<u8>>
}

#[derive(Serialize, Deserialize)]
pub struct UpdateBodyPayload {
    pub body: String,
    pub post_id: String
}

#[derive(Serialize, Deserialize)]
pub struct ApprovePostPayload {
    pub post_id: String,
    pub approval_state: bool
}