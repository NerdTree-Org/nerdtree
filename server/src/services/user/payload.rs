use crate::db::user::models::UserModel;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize)]
pub struct GetUserByIdPayload {
    pub id: String,
}

#[derive(Deserialize)]
pub struct GetUserByEmailPayload {
    pub email: String,
}

#[derive(Deserialize)]
pub struct GetUserByUsernamePayload {
    pub username: String,
}

#[derive(Serialize)]
pub struct QueriedUser {
    pub id: uuid::Uuid,
    pub username: String,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub profile_pic: Option<String>,
    pub is_admin: bool,
    pub facebook_id: String,
    pub joined_date: chrono::NaiveDateTime,
}

impl QueriedUser {
    pub fn new(unsafe_user: UserModel) -> Self {
        QueriedUser {
            id: unsafe_user.id,
            firstname: unsafe_user.firstname,
            lastname: unsafe_user.lastname,
            email: unsafe_user.email,
            username: unsafe_user.username,
            facebook_id: unsafe_user.facebook_id,
            is_admin: unsafe_user.is_admin,
            profile_pic: unsafe_user.profile_pic,
            joined_date: unsafe_user.joined_date,
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct UpdateFirstnamePayload {
    #[validate(length(min = 3, max = 255))]
    pub firstname: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateLastnamePayload {
    #[validate(length(min = 3, max = 255))]
    pub lastname: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdateEmailPayload {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct UpdatePasswordPayload {
    pub old_password: String,

    #[validate(length(min = 8, max = 80))]
    pub new_password: String,
}

#[derive(Serialize)]
pub struct StatusPayload {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct DeleteUserPayload {
    pub user_id: String,
}
