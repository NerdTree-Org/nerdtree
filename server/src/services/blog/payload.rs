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