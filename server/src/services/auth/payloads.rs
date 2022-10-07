use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 3, max = 25))]
    pub username: String,

    #[validate(length(min = 3, max = 255))]
    pub firstname: String,

    #[validate(length(min = 3, max = 255))]
    pub lastname: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 80))]
    pub password: String,

    #[validate(length(max = 80))]
    pub facebook_id: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatusPayload {
    pub success: bool,
}

#[derive(Serialize)]
pub struct TokensPayload {
    pub refreshtoken: String,
    pub accesstoken: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshAccessTokenPayload {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshAccessTokenReturnPayload {
    pub access_token: String,
}

#[derive(Deserialize)]
pub struct PasswordResetRequestPayload {
    pub email: String,
}

#[derive(Deserialize)]
pub struct PasswordResetTokenPayload {
    pub user_id: String,
    pub reset_token: String,
    pub password: String,
}
