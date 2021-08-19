use actix_web::web::{Json, Data};
use crate::services::comment::payload::{GetCommentsByPostPayload, PaginatedCommentsReturnPayload, GetCommentsByUserPayload};
use crate::db::Pool;
use actix_web::Responder;
use crate::errors::Errors;
use crate::db::comment::query::{get_comments_by_post, get_comments_by_author};
use uuid::Uuid;
use std::str::FromStr;
use crate::paginated_vec::PaginatedVec;
use actix_web_validator::Json as Validate;

pub async fn get_comments_by_post_handler(
    payload: Validate<GetCommentsByPostPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let comments = get_comments_by_post(&Uuid::from_str(&payload.post_id)
        .map_err(|e| Errors::BadRequest(e.to_string()))?, &conn_pool)?;

    let paginated_comments = PaginatedVec::from_vec(&comments, payload.per_page);
    let page = paginated_comments.page(payload.page);

    return if page.is_none() {
        Ok(Json(PaginatedCommentsReturnPayload {
            current_page: payload.page,
            max_page: paginated_comments.get_max_pages(),
            page: Vec::new()
        }))
    } else {
        Ok(Json(PaginatedCommentsReturnPayload {
            current_page: page.unwrap().0 + 1,
            max_page: paginated_comments.get_max_pages(),
            page: page.unwrap().1.into_iter().map(|v| v.clone().clone()).collect()
        }))
    }
}

pub async fn get_comments_by_user_handler(
    payload: Validate<GetCommentsByUserPayload>,
    conn_pool: Data<Pool>
) -> Result<impl Responder, Errors> {
    let comments = get_comments_by_author(&Uuid::from_str(&payload.author_id)
        .map_err(|e| Errors::BadRequest(e.to_string()))?, &conn_pool)?;

    let paginated_comments = PaginatedVec::from_vec(&comments, payload.per_page);
    let page = paginated_comments.page(payload.page);

    return if page.is_none() {
        Ok(Json(PaginatedCommentsReturnPayload {
            current_page: payload.page,
            max_page: paginated_comments.get_max_pages(),
            page: Vec::new()
        }))
    } else {
        Ok(Json(PaginatedCommentsReturnPayload {
            current_page: page.unwrap().0 + 1,
            max_page: paginated_comments.get_max_pages(),
            page: page.unwrap().1.into_iter().map(|v| v.clone().clone()).collect()
        }))
    }
}