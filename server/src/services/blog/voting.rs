use crate::db::voting::mutation::{add_downvote, add_upvote, remove_downvote, remove_upvote};
use crate::db::voting::query::{
    get_downvote_by_user_for_post, get_downvotes_of_post, get_upvote_by_user_for_post,
    get_upvotes_of_post,
};
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::login_required::LoginRequired;
use crate::services::blog::payload::{
    AddUpvotePayload, GetUserVoteForPostPayload, GetUserVoteForPostReturnPayload, GetVotesPayload,
    GetVotesReturnPayload, StatusPayload,
};
use actix_web::web::{Data, Json};
use actix_web::Responder;
use std::str::FromStr;
use uuid::Uuid;

pub async fn get_user_vote_for_post(
    payload: Json<GetUserVoteForPostPayload>,
    conn_pool: Data<Pool>,
    user: LoginRequired,
) -> Result<impl Responder, Errors> {
    let post_id =
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let user_id = user.user.id;

    let mut vote: i8 = 0;
    if !get_upvote_by_user_for_post(post_id, user_id, &conn_pool)?.is_empty() {
        vote += 1;
    } else if !get_downvote_by_user_for_post(post_id, user_id, &conn_pool)?.is_empty() {
        vote -= 1;
    }

    Ok(Json(GetUserVoteForPostReturnPayload { vote }))
}

pub async fn add_upvote_handler(
    payload: Json<AddUpvotePayload>,
    conn_pool: Data<Pool>,
    user: LoginRequired,
) -> Result<impl Responder, Errors> {
    let post_id =
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let user_id = user.user.id;

    // check if there's already an upvote
    if !get_upvote_by_user_for_post(post_id, user_id, &conn_pool)?.is_empty() {
        return Ok(Json(
            remove_upvote(user_id, post_id, &conn_pool).map(|_| StatusPayload { success: true })?,
        ));
    }

    if !get_downvote_by_user_for_post(post_id, user_id, &conn_pool)?.is_empty() {
        remove_downvote(user_id, post_id, &conn_pool)?;
    }

    add_upvote(user_id, post_id, &conn_pool)?;
    Ok(Json(StatusPayload { success: true }))
}

pub async fn add_downvote_handler(
    payload: Json<AddUpvotePayload>,
    conn_pool: Data<Pool>,
    user: LoginRequired,
) -> Result<impl Responder, Errors> {
    let post_id =
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let user_id = user.user.id;

    // check if there's already a downvote
    if !get_downvote_by_user_for_post(post_id, user_id, &conn_pool)?.is_empty() {
        return Ok(Json(
            remove_downvote(user_id, post_id, &conn_pool)
                .map(|_| StatusPayload { success: true })?,
        ));
    }

    if !get_upvote_by_user_for_post(post_id, user_id, &conn_pool)?.is_empty() {
        remove_upvote(user_id, post_id, &conn_pool)?;
    }

    add_downvote(user_id, post_id, &conn_pool)?;
    Ok(Json(StatusPayload { success: true }))
}

pub async fn get_votes(
    payload: Json<GetVotesPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let post_id =
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let upvotes = get_upvotes_of_post(post_id, &conn_pool)?.len() as i64;
    let downvotes = get_downvotes_of_post(post_id, &conn_pool)?.len() as i64;

    Ok(Json(GetVotesReturnPayload {
        votes: upvotes - downvotes,
    }))
}
