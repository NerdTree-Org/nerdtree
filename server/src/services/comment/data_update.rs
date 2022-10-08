use crate::db::comment::mutation::{delete_comment, edit_comment, insert_comment};
use crate::db::comment::query::get_comments_by_uuid;
use crate::db::post::query::get_post_by_uuid;
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::login_required::LoginRequired;
use crate::services::comment::payload::{
    DeleteCommentPayload, EditCommentPayload, NewCommentPayload,
};
use actix_web::web::{Data, Json};
use actix_web::Responder;
use std::str::FromStr;
use uuid::Uuid;

pub async fn new_comment_handler(
    payload: Json<NewCommentPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    // get post id
    let post_id =
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;

    // check if post exists
    let post = get_post_by_uuid(post_id, false, &conn_pool)?;
    if post.is_empty() {
        return Err(Errors::BadRequest(String::from("Post doesn't exist!")));
    }

    // insert comment into db
    Ok(Json(insert_comment(
        &post_id,
        &user.user.id,
        &payload.body,
        &conn_pool,
    )?))
}

/// Only allow if the user is actually the owner of the comment or
/// the user is the admin
pub async fn edit_comment_handler(
    payload: Json<EditCommentPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    // get the comment
    let comment = get_comments_by_uuid(
        &Uuid::from_str(&payload.comment_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        &conn_pool,
    )?;

    if comment.is_empty() {
        return Err(Errors::BadRequest(String::from("No such comment")));
    }
    let comment = comment[0].clone();

    // check if user is allowed to edit the comment
    match comment.author_id {
        Some(id) => {
            if id != user.user.id && !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
        None => {
            if !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
    }

    // edit the comment
    Ok(Json(edit_comment(&comment.id, &payload.body, &conn_pool)?))
}

pub async fn delete_comment_handler(
    payload: Json<DeleteCommentPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let comment = get_comments_by_uuid(
        &Uuid::from_str(&payload.comment_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        &conn_pool,
    )?;

    if comment.is_empty() {
        return Err(Errors::BadRequest(String::from("No such comment")));
    }
    let comment = comment[0].clone();

    // check if user is allowed to edit the comment
    match comment.author_id {
        Some(id) => {
            if id != user.user.id && !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
        None => {
            if !user.user.is_admin {
                return Err(Errors::AccessForbidden);
            }
        }
    }

    // delete the comment
    Ok(Json(delete_comment(&comment.id, &conn_pool)?))
}
