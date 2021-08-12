use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct NewPostPayload {
    #[validate(length(min = 3, max = 255))]
    pub title: String,

    pub body: String,

    #[validate(length(min = 40, max = 41))]
    pub thumbnail: String,
}
