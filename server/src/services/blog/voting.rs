use actix_web::web::{Data, Json};
use crate::services::blog::payload::{AddUpvotePayload, StatusPayload, GetVotesPayload, GetVotesReturnPayload};
use crate::db::Pool;
use crate::errors::Errors;
use uuid::Uuid;
use std::str::FromStr;
use crate::db::voting::query::{get_upvote_by_user_for_post, get_downvote_by_user_for_post, get_upvotes_of_post, get_downvotes_of_post};
use crate::db::voting::mutation::{remove_upvote, remove_downvote, add_upvote, add_downvote};
use crate::guards::login_required::LoginRequired;
use actix_web::Responder;

/// Checks if user already "upvoted" the post
/// If they did, just fucking remove the upvote and return from the foking route
/// If they didn't upvote but did downvote, remove the downvote and then continue
/// and add a foking upvote into the database
/// and pretend like nothing happened
pub async fn add_upvote_handler(
    payload: Json<AddUpvotePayload>,
    conn_pool: Data<Pool>,
    user: LoginRequired
) -> Result<impl Responder, Errors> {
    // convert the ids from
    let post_id = Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let user_id = user.user.id.clone();

    // check if there's already an upvote
    if get_upvote_by_user_for_post(post_id, user_id, &conn_pool)?.len() != 0 {
        return Ok(Json(remove_upvote(user_id, post_id, &conn_pool).map(|_| StatusPayload { success: true })?));
    }

    // there's no foking upvote
    // check for downvote
    //       ||
    //      \\//
    //       \/
    if get_downvote_by_user_for_post(post_id, user_id, &conn_pool)?.len() != 0 {
        remove_downvote(user_id, post_id, &conn_pool)?;
    }

    // now foking add an upvote
    // ahhh, wasting so much time just foking commenting. This is the most I've commented on code
    add_upvote(user_id, post_id, &conn_pool)?;
    Ok(Json(StatusPayload {
        success: true
    }))
}

// replace("upvote").with("downvote")
//      .and(replace("downvote").with("upvote"))
/// Checks if user already "downvoted" the post
/// If they did, just fucking remove the downvote and return from the foking route
/// If they didn't downvote but did upvote, remove the upvote and then continue
/// and add a foking downvote into the database
/// and pretend like nothing happened
pub async fn add_downvote_handler(
    payload: Json<AddUpvotePayload>,
    conn_pool: Data<Pool>,
    user: LoginRequired
) -> Result<impl Responder, Errors> {
    // convert the ids from
    let post_id = Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let user_id = user.user.id.clone();

    // check if there's already a downvote
    if get_downvote_by_user_for_post(post_id, user_id, &conn_pool)?.len() != 0 {
        return Ok(Json(remove_downvote(user_id, post_id, &conn_pool).map(|_| StatusPayload { success: true })?));
    }

    // there's no foking downvote
    // check for upvote
    //
    //      /\
    //     //\\
    //      ||
    if get_upvote_by_user_for_post(post_id, user_id, &conn_pool)?.len() != 0 {
        remove_upvote(user_id, post_id, &conn_pool)?;
    }

    // now foking add an downvote
    // ahhh, wasting so much time just foking commenting. This is the most I've commented on code
    add_downvote(user_id, post_id, &conn_pool)?;
    Ok(Json(StatusPayload {
        success: true
    }))
}

pub async fn get_votes(
    payload: Json<GetVotesPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let post_id = Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?;
    let upvotes = get_upvotes_of_post(post_id, &conn_pool)?.len() as i64;
    let downvotes = get_downvotes_of_post(post_id, &conn_pool)?.len() as i64;

    return Ok(Json(GetVotesReturnPayload {
        votes: upvotes - downvotes
    }))
}