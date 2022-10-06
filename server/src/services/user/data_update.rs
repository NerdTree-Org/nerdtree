use crate::db::user::mutation::{
    delete_user, update_user_email, update_user_firstname, update_user_lastname,
    update_user_password, update_user_profile_pic,
};
use crate::db::user::query::get_users_by_email;
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::admin_only::AdminOnly;
use crate::guards::login_required::LoginRequired;
use crate::services::user::payload::{
    DeleteUserPayload, QueriedUser, StatusPayload, UpdateEmailPayload, UpdateFirstnamePayload,
    UpdateLastnamePayload, UpdatePasswordPayload,
};
use actix_multipart::Multipart;
use actix_web::{
    web::{Data, Json},
    Responder,
};
use actix_web_validator::Json as validate;
use argon2::Config;
use futures_util::{StreamExt, TryStreamExt};
use rand::RngCore;
use std::str::FromStr;
use uuid::Uuid;

pub async fn update_firstname_handler(
    user: LoginRequired,
    payload: validate<UpdateFirstnamePayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    update_user_firstname(&user.user.id, &payload.firstname, &conn_pool)
        .map_or_else(Err, |_| Ok(Json(StatusPayload { success: true })))
}

pub async fn update_lastname_handler(
    user: LoginRequired,
    payload: validate<UpdateLastnamePayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    update_user_lastname(&user.user.id, &payload.lastname, &conn_pool)
        .map_or_else(Err, |_| Ok(Json(StatusPayload { success: true })))
}

pub async fn update_email_handler(
    user: LoginRequired,
    payload: validate<UpdateEmailPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    // first check if there's other users with same email
    let users = get_users_by_email(&payload.email, &conn_pool)?;
    if !users.is_empty() && users[0].id != user.user.id {
        return Err(Errors::BadRequest("Invalid email".to_string()));
    }

    update_user_email(&user.user.id, &payload.email, &conn_pool)
        .map_or_else(Err, |_| Ok(Json(StatusPayload { success: true })))
}

pub async fn update_password_handler(
    user: LoginRequired,
    payload: validate<UpdatePasswordPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    if argon2::verify_encoded(&user.user.password, payload.old_password.as_bytes())
        .map_err(|_| Errors::InternalServerError)?
    {
        let config = Config::default();
        let mut salt = [0u8; 30];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut salt);

        let hash = argon2::hash_encoded(payload.new_password.as_bytes(), &salt, &config)
            .map_err(|_| Errors::InternalServerError)?;
        update_user_password(&user.user.id, &hash, &conn_pool)
            .map_or_else(Err, |_| Ok(Json(StatusPayload { success: true })))
    } else {
        Err(Errors::BadRequest("Wrong Password".to_string()))
    }
}

pub async fn delete_user_handler(
    payload: Json<DeleteUserPayload>,
    _: AdminOnly,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let user_id = Uuid::from_str(&payload.user_id)
        .map_err(|_| Errors::BadRequest("Invalid uuid".to_string()))?;
    delete_user(&user_id, &conn_pool).map(|_| Json(StatusPayload { success: true }))
}

pub async fn upload_profile_pic_handler(
    mut payload: Multipart,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    // only care about the first field, cause we don't give a fuck
    let mut field = payload
        .try_next()
        .await
        .map_err(|e| Errors::BadRequest(e.to_string()))?
        .unwrap();
    let mut file_bytes: Vec<u8> = vec![];

    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        file_bytes.extend(data.to_vec());
    }

    // check file size
    // as u8 is 8 bits or 1 byte, we can just count the length of the bytes array
    if file_bytes.len() > 1024 * 2048 {
        return Err(Errors::BadRequest("Too big file".to_string()));
    }

    // write the image to image dir
    let img_filename = Uuid::new_v4().to_string() + ".png";
    let img_dir = std::env::var("IMAGE_PATH").unwrap();
    let img_dir = std::path::Path::new(&img_dir);

    // we should have all the bytes collected
    image::load_from_memory(&file_bytes)
        .map_err(|e| Errors::BadRequest(e.to_string()))?
        .save(img_dir.join(&img_filename))
        .map_err(|_| Errors::InternalServerError)?;

    // try deleting the previous avatar
    if let Some(previous_pic) = user.user.profile_pic {
        let _ = std::fs::remove_file(img_dir.join(previous_pic));
    }

    // try update the avatar in db
    Ok(Json(QueriedUser::new(update_user_profile_pic(
        &user.user.id,
        &img_filename,
        &conn_pool,
    )?)))
}
