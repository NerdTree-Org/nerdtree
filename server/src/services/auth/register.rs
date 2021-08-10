use actix_web_validator::Json;
use actix_web::{web::Json as actix_json, Responder, web::Data};
use crate::errors::Errors;
use crate::db::Pool;
use crate::services::auth::payloads::RegisterRequest;
use argon2::{self, Config};
use crate::db::user::query::is_unique_user;
use super::payloads::StatusPayload;
use rand::RngCore;
use crate::db::user::mutation::insert_user;

pub async fn register_handler(
    conn_pool: Data<Pool>,
    payload: Json<RegisterRequest>
) -> Result<impl Responder, Errors> {
    // check for existing user
    if !is_unique_user(&payload.email, &payload.username, &conn_pool)? {
        return Err(Errors::BadRequest("User is not unique!".to_string()));
    }

    let config = Config::default();
    let mut salt = [0u8; 30];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut salt);

    let hash = argon2::hash_encoded(payload.password.clone().as_ref(), &salt, &config).map_err(|_| Errors::InternalServerError)?;

    insert_user(&payload.username, &payload.firstname, &payload.lastname, &hash, &payload.email, &payload.facebook_id,&conn_pool)?;

    Ok(actix_json(StatusPayload {
        success: true
    }))
}