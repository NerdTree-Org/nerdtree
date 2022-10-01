use crate::db::user::mutation::invalidate_discord_token;
use crate::db::user::query::get_users_by_username;
use crate::db::Pool;
use crate::errors::Errors;
use actix_web::web::{Data, Json};
use actix_web::Responder;
/// Not gonna make it complicated by creating 3 files to make 1 route
/// Just gonna add the payload and stuff here. It doesn't require any access to db other than
/// the user thing....
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DiscordVerifyPayload {
    pub username: String,
    pub discord_token: String,
    pub discord_bot_token: String,
}

#[derive(Serialize, Deserialize)]
struct StatusPayload {
    pub success: bool,
}

pub async fn verify_token(
    payload: Json<DiscordVerifyPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    // check if user exists with that discord token, if exists, then return true
    let allowed_bot_token = std::env::var("DISCORD_BOT_TOKEN").unwrap();

    // check if sender is allowed to make this request
    if allowed_bot_token != payload.discord_bot_token {
        return Err(Errors::AccessForbidden);
    }

    // find user with the provided username
    let user = get_users_by_username(&payload.username, &conn_pool)?;
    if user.is_empty() {
        return Err(Errors::BadRequest("No such user found with given nickname. Please make sure that your discord username is the exact same one as the one you provided in registration form".to_string()));
    }
    let user = user[0].clone();

    if user.discord_token.to_string() != payload.discord_token || user.is_discord_token_used {
        Err(Errors::BadRequest("Invalid discord token!".to_string()))
    } else {
        // invalidate the token and return success thing
        invalidate_discord_token(&user.id, &conn_pool)?;

        Ok(Json(StatusPayload { success: true }))
    }
}
