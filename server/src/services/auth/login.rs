use crate::db::user::query::get_users_by_username;
use crate::db::Pool;
use crate::errors::Errors;
use crate::jwt::sign::{generate_accesstoken, generate_refreshtoken};
use crate::services::auth::payloads::{LoginRequest, TokensPayload};
use actix_web::{
    web::{Data, Json},
    Responder,
};

pub async fn login_handler(
    payload: Json<LoginRequest>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let users = get_users_by_username(&payload.username, &conn_pool)?;
    if users.is_empty() {
        return Err(Errors::BadRequest("Wrong Credentials".to_string()));
    }

    let user = users[0].clone();

    let hashed_password = user.password;
    let matches = argon2::verify_encoded(&hashed_password, payload.password.as_ref())
        .map_err(|_| Errors::InternalServerError)?;

    if !matches {
        return Err(Errors::BadRequest("Wrong Credentials".to_string()));
    }

    let accesstoken = generate_accesstoken(user.id).map_err(|_| Errors::InternalServerError)?;
    let refreshtoken = generate_refreshtoken(user.id).map_err(|_| Errors::InternalServerError)?;

    Ok(Json(TokensPayload {
        accesstoken,
        refreshtoken,
    }))
}
