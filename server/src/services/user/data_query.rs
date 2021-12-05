use crate::db::user::query::{get_user_by_id, get_users_by_email, get_users_by_username};
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::login_required::LoginRequired;
use crate::services::user::payload::{
    GetUserByEmailPayload, GetUserByIdPayload, GetUserByUsernamePayload, QueriedUser,
};
use actix_web::{
    web::{Data, Json},
    Responder,
};
use std::str::FromStr;
use uuid::Uuid;

pub async fn get_user_by_id_handler(
    payload: Json<GetUserByIdPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let user_id =
        Uuid::from_str(&payload.id).map_err(|_| Errors::BadRequest("Invalid uuid".to_string()))?;
    let user = get_user_by_id(&user_id, &conn_pool)?;

    if user.is_empty() {
        return Err(Errors::BadRequest("No such user".to_string()));
    }

    Ok(Json(QueriedUser::new(user[0].clone())))
}

pub async fn get_user_by_email_handler(
    payload: Json<GetUserByEmailPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let user = get_users_by_email(&payload.email, &conn_pool)?;
    if user.is_empty() {
        return Err(Errors::BadRequest("No such user".to_string()));
    }

    Ok(Json(QueriedUser::new(user[0].clone())))
}

pub async fn get_user_by_username_handler(
    payload: Json<GetUserByUsernamePayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let user = get_users_by_username(&payload.username, &conn_pool)?;
    if user.is_empty() {
        return Err(Errors::BadRequest("No such user".to_string()));
    }

    Ok(Json(QueriedUser::new(user[0].clone())))
}

pub async fn get_current_user_handler(user: LoginRequired) -> impl Responder {
    Json(QueriedUser::new(user.user))
}

pub async fn get_discord_token(user: LoginRequired) -> impl Responder {
    user.user.discord_token.to_string()
}
