use crate::db::post::mutation::insert_post;
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::login_required::LoginRequired;
use crate::services::blog::payload::NewPostPayload;
use actix_web::{
    web::{Data, Json},
    Responder,
};
use actix_web_validator::Json as Validate;

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
