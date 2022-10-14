use crate::db::post::models::PostModel;
use crate::db::post::query::{get_all_posts, get_post_by_uuid, get_posts_by_author_id};
use crate::db::Pool;
use crate::errors::Errors;
use crate::guards::login_required::LoginRequired;
use crate::paginated_vec::PaginatedVec;
use crate::services::blog::payload::{
    GetPostByIdPayload, GetPostsByAuthorId, PaginatePostsPayload, PaginatePostsReturnPayload,
};
use actix_web::web::{Data, Json};
use actix_web::Responder;
use actix_web_validator::Json as Validate;
use std::str::FromStr;
use uuid::Uuid;

pub async fn paginate_posts_handler(
    payload: Validate<PaginatePostsPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let posts = get_all_posts(&conn_pool, false)?;
    let pg_vec = PaginatedVec::from_vec(&posts, payload.per_page);
    let page = pg_vec.page(payload.page - 1);

    // I don't know what the fuck I'm doing. Probably a piece of code that'll make this goddamn thing slower
    return if page.is_none() {
        Ok(Json(PaginatePostsReturnPayload {
            current_page: payload.page,
            max_page: pg_vec.get_max_pages(),
            page: Vec::new(),
        }))
    } else {
        Ok(Json(PaginatePostsReturnPayload {
            current_page: page.unwrap().0 + 1,
            max_page: pg_vec.get_max_pages(),
            page: page.unwrap().1.iter().map(|v| (*v).clone()).collect(),
        }))
    };
}

pub async fn paginate_posts_handler_including_unapproved(
    payload: Validate<PaginatePostsPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let posts = get_all_posts(&conn_pool, true)?;
    let pg_vec = PaginatedVec::from_vec(&posts, payload.per_page);
    let page = pg_vec.page(payload.page - 1);

    // I don't know what the fuck I'm doing. Probably a piece of code that'll make this goddamn thing slower
    return if page.is_none() {
        Ok(Json(PaginatePostsReturnPayload {
            current_page: payload.page,
            max_page: pg_vec.get_max_pages(),
            page: Vec::new(),
        }))
    } else {
        Ok(Json(PaginatePostsReturnPayload {
            current_page: page.unwrap().0 + 1,
            max_page: pg_vec.get_max_pages(),
            page: page
                .unwrap()
                .1
                .iter()
                .map(|v| (*v).clone())
                .filter(|p| p.post_author == Some(user.user.id) || user.user.is_admin)
                .collect(),
        }))
    };
}

pub async fn get_post_by_id_handler(
    payload: Json<GetPostByIdPayload>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    Ok(Json(
        get_post_by_uuid(
            Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
            false,
            &conn_pool,
        )?
        .get(0)
        .ok_or_else(|| Errors::BadRequest(String::from("No such post!")))?
        .clone(),
    ))
}

pub async fn get_post_by_id_handler_including_unapproved(
    payload: Json<GetPostByIdPayload>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let post = get_post_by_uuid(
        Uuid::from_str(&payload.post_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?
    .get(0)
    .ok_or_else(|| Errors::BadRequest(String::from("No such post!")))?
    .clone();

    if post.post_author != Some(user.user.id) && !user.user.is_admin {
        return Err(Errors::BadRequest(String::from("No such post!")));
    }

    Ok(Json(post))
}

pub async fn get_posts_by_author_id_handler(
    payload: Json<GetPostsByAuthorId>,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let posts = get_posts_by_author_id(
        Uuid::from_str(&payload.author_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        false,
        &conn_pool,
    )?;
    Ok(Json(posts))
}

pub async fn get_posts_by_author_id_handler_including_unapproved(
    payload: Json<GetPostsByAuthorId>,
    user: LoginRequired,
    conn_pool: Data<Pool>,
) -> Result<impl Responder, Errors> {
    let posts = get_posts_by_author_id(
        Uuid::from_str(&payload.author_id).map_err(|e| Errors::BadRequest(e.to_string()))?,
        true,
        &conn_pool,
    )?
    .iter()
    .filter(|p| p.post_author == Some(user.user.id) || user.user.is_admin)
    .map(|p| p.clone())
    .collect::<Vec<PostModel>>();
    Ok(Json(posts))
}
