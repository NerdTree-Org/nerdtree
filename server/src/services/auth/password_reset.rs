use crate::db::user::mutation::update_user_password;
use crate::db::user::query::{get_user_by_id, get_users_by_email};
use crate::db::Pool;
use crate::email::send_email;
use crate::errors::Errors;
use crate::jwt::claims::TokenType;
use crate::jwt::sign::generate_passwordresettoken;
use crate::jwt::verify::{decode_without_secret, verify_token_with_custom_secret};
use crate::services::auth::payloads::{
    PasswordResetRequestPayload, PasswordResetTokenPayload, StatusPayload,
};
use actix_web::{
    web::{Data, Json},
    Responder,
};
use argon2::Config;
use rand::RngCore;
use std::str::FromStr;

pub async fn password_reset_request_handler(
    payload: Json<PasswordResetRequestPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let users = get_users_by_email(&payload.email, &conn_pool)?;

    if users.is_empty() {
        // no such user with email
        return Err(Errors::BadRequest("No such user".to_string()));
    }

    let user = users[0].clone();
    let password_reset_token = generate_passwordresettoken(
        user.id,
        &(user.password + &std::env::var("JWT_SECRET_KEY").unwrap()),
    )
    .map_err(|_| Errors::InternalServerError)?;

    let email_templ = format!("We've received a request for recovering password for user: {}. Copy this token and paste it into correct place. DO NOT SHARE THIS TOKEN! Token: {}", user.username, password_reset_token);

    if !send_email(
        "Password Reset Request for NerdTree Account",
        &email_templ,
        &user.email,
    ) {
        Err(Errors::InternalServerError)
    } else {
        Ok(Json(StatusPayload { success: true }))
    }
}

pub async fn password_reset_token_handler(
    payload: Json<PasswordResetTokenPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let decoded_token =
        decode_without_secret(&payload.reset_token).map_err(|_| Errors::InternalServerError)?;

    // check if token type is password reset token
    match decoded_token.token_type {
        TokenType::PasswordResetToken => (),
        _ => return Err(Errors::BadRequest("Invalid token".to_string())),
    };

    let users = get_user_by_id(
        &uuid::Uuid::from_str(&decoded_token.id).unwrap(),
        &conn_pool,
    )?;
    if users.is_empty() {
        // no such user
        return Err(Errors::BadRequest("Malformed token".to_string()));
    }
    let user = users[0].clone();

    // now verify the token
    verify_token_with_custom_secret(
        &payload.reset_token,
        &(user.password + &std::env::var("JWT_SECRET_KEY").unwrap()),
    )
    .map_err(|_| Errors::BadRequest("Malformed token".to_string()))?;

    // now change the password
    let config = Config::default();
    let mut salt = [0u8; 30];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);
    let hash = argon2::hash_encoded(payload.password.clone().as_ref(), &salt, &config)
        .map_err(|_| Errors::InternalServerError)?;

    Ok(Json(StatusPayload {
        success: update_user_password(&user.id, &hash, &conn_pool).is_ok(),
    }))
}
