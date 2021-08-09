use crate::guards::login_required::LoginRequired;
use actix_web_validator::Json as validate;
use actix_web::{web::{Json, Data}, Responder};
use crate::services::user::payload::{UpdateFirstnamePayload, StatusPayload, UpdateLastnamePayload, UpdateEmailPayload, UpdatePasswordPayload, DeleteUserPayload};
use crate::db::Pool;
use crate::errors::Errors;
use crate::db::user::mutation::{update_user_firstname, update_user_lastname, update_user_email, update_user_password, delete_user};
use rand::RngCore;
use argon2::Config;
use uuid::Uuid;
use std::str::FromStr;
use crate::guards::admin_only::AdminOnly;
use crate::db::user::query::get_users_by_email;

pub async fn update_firstname_handler(
    user: LoginRequired,
    payload: validate<UpdateFirstnamePayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    update_user_firstname(&user.user.id, &payload.firstname, &conn_pool).map_or_else(|e| Err(e), |_| Ok(Json(StatusPayload {
        success: true
    })))
}

pub async fn update_lastname_handler(
    user: LoginRequired,
    payload: validate<UpdateLastnamePayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    update_user_lastname(&user.user.id, &payload.lastname, &conn_pool).map_or_else(|e| Err(e), |_| Ok(Json(StatusPayload {
        success: true
    })))
}

pub async fn update_email_handler(
    user: LoginRequired,
    payload: validate<UpdateEmailPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    // first check if there's other users with same email
    let users = get_users_by_email(&payload.email, &conn_pool)?;
    if users.len() >= 1 {
        if users[0].id != user.user.id {
            return Err(Errors::BadRequest("Invalid email"))
        }
    }

    update_user_email(&user.user.id, &payload.email, &conn_pool).map_or_else(|e| Err(e), |_| Ok(Json(StatusPayload {
        success: true
    })))
}

pub async fn update_password_handler(
    user: LoginRequired,
    payload: validate<UpdatePasswordPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    if argon2::verify_encoded(&user.user.password, payload.old_password.as_bytes()).map_err(|_| Errors::InternalServerError)? {
        let config = Config::default();
        let mut salt = [0u8; 30];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut salt);

        let hash = argon2::hash_encoded(payload.new_password.as_bytes(), &salt, &config).map_err(|_| Errors::InternalServerError)?;
        update_user_password(&user.user.id, &hash, &conn_pool).map_or_else(|e| Err(e), |_| Ok(Json(StatusPayload {
            success: true
        })))
    }
    else {
        Err(Errors::BadRequest("Wrong Password"))
    }
}

pub async fn delete_user_handler(
    payload: Json<DeleteUserPayload>,
    _: AdminOnly,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let user_id = Uuid::from_str(&payload.user_id).map_err(|_| Errors::BadRequest("Invalid uuid"))?;
    delete_user(&user_id, &conn_pool).map(|_| Json(StatusPayload {
        success: true
    }))
}