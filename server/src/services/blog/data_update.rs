use crate::db::post::mutation::{
    delete_post, insert_post, update_approval_status, update_post_body, update_thumbnail,
    update_title,
};
use crate::db::post::query::get_post_by_uuid;
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::admin_only::AdminOnly;
use crate::guards::login_required::LoginRequired;
use crate::services::blog::payload::{
    ApprovePostPayload, DeletePostPayload, NewPostPayload, StatusPayload, UpdateBodyPayload,
    UpdateTitlePayload, UploadThumbnailForm,
};
use actix_web::{
    web::{Data, Json},
    Responder,
};
use actix_web_validator::Json as Validate;
use std::str::FromStr;
use uuid::Uuid;

pub async fn new_post_handler(
    payload: Validate<NewPostPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    // insert new post
    Ok(Json(insert_post(
        &payload.title,
        &payload.body,
        &user.user.id,
        &conn_pool,
    )?))
}

pub async fn upload_thumbnail_thumbnail(
    payload: awmpde::Multipart<UploadThumbnailForm>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let UploadThumbnailForm {
        thumbnail: awmpde::File { inner, .. },
        post_id,
    } = payload
        .into_inner()
        .await
        .map_err(|e| Errors::BadRequest(e.to_string()))?;

    let post = get_post_by_uuid(
        Uuid::from_str(&post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?;

    // check if post exists
    if post.is_empty() {
        return Err(Errors::BadRequest(String::from("No such post")));
    }

    let post = post[0].clone();

    // check if the user is the actual author
    match post.post_author {
        Some(id) => {
            if !id.eq(&user.user.id) && !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
        None => {
            if !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
    }

    // check size (max 10MB)
    if inner.len() > 1024 * 10240 {
        return Err(Errors::BadRequest(String::from("Too big file")));
    }

    let image_path = std::env::var("IMAGE_PATH").unwrap();
    let img_filename = Uuid::new_v4().to_string() + ".png";
    let image_path = std::path::Path::new(&image_path);

    image::load_from_memory(&inner)
        .map_err(|e| Errors::BadRequest(e.to_string()))?
        .save(image_path.join(&img_filename))
        .map_err(|_| Errors::InternalServerError)?;

    // try deleting the previous thumbnail
    if let Some(thumbnail) = post.thumbnail {
        let _ = std::fs::remove_file(image_path.join(thumbnail));
    }

    Ok(Json(update_thumbnail(&img_filename, &post.id, &conn_pool)?))
}

pub async fn update_post_body_handler(
    payload: Json<UpdateBodyPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let post = get_post_by_uuid(
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?;
    if post.is_empty() {
        return Err(Errors::BadRequest(String::from("No such post!")));
    }

    let post = post[0].clone();

    // check if user is allowed to do that
    match post.post_author {
        Some(id) => {
            if !id.eq(&user.user.id) && !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
        None => {
            if !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
    }

    Ok(Json(update_post_body(&payload.body, &post.id, &conn_pool)?))
}

pub async fn approve_post_handler(
    payload: Json<ApprovePostPayload>,
    _: AdminOnly,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let post = get_post_by_uuid(
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?;
    if post.is_empty() {
        return Err(Errors::BadRequest(String::from("No such post!")));
    }
    let post = post[0].clone();

    Ok(Json(update_approval_status(
        payload.approval_state,
        &post.id,
        &conn_pool,
    )?))
}

pub async fn delete_post_handler(
    payload: Json<DeletePostPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let post = get_post_by_uuid(
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?;
    if post.is_empty() {
        return Err(Errors::BadRequest(String::from("No such post!")));
    }
    let post = post[0].clone();

    // check if user is allowed
    if post.post_author.is_some()
        && post.post_author.unwrap() != user.user.id
        && !user.user.is_admin
    {
        Err(Errors::AccessForbidden)
    } else {
        // delete the post
        delete_post(&post.id, &conn_pool).map(|_| Json(StatusPayload { success: true }))
    }
}

pub async fn update_post_title_handler(
    payload: Validate<UpdateTitlePayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let post = get_post_by_uuid(
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?;
    if post.is_empty() {
        return Err(Errors::BadRequest(String::from("No such post!")));
    }
    let post = post[0].clone();

    // check if user is allowed
    if post.post_author.is_some()
        && post.post_author.unwrap() != user.user.id
        && !user.user.is_admin
    {
        Err(Errors::AccessForbidden)
    } else {
        Ok(Json(update_title(
            &payload.new_title,
            &Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
            &conn_pool,
        )?))
    }
}
