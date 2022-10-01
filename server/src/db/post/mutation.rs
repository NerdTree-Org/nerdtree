use crate::db::post::models::{NewPost, PostModel};
use crate::db::schema::posts::dsl::*;
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::prelude::*;
use uuid::Uuid;

pub fn insert_post(
    post_title: &str,
    post_body: &str,
    post_creator: &Uuid,
    conn_pool: &Pool,
) -> Result<PostModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_post = NewPost {
        title: post_title,
        body: post_body,
        post_author: post_creator,
    };

    diesel::insert_into(posts)
        .values(new_post)
        .get_result::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn update_thumbnail(
    post_thumbnail: &str,
    post_id: &Uuid,
    conn_pool: &Pool,
) -> Result<PostModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(thumbnail.eq(post_thumbnail))
        .get_result::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn update_post_body(
    post_body: &str,
    post_id: &Uuid,
    conn_pool: &Pool,
) -> Result<PostModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(body.eq(post_body))
        .get_result::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn update_approval_status(
    post_approval_state: bool,
    post_id: &Uuid,
    conn_pool: &Pool,
) -> Result<PostModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(approval_date.eq(chrono::Utc::now().naive_utc()))
        .get_result::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)?;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(is_approved.eq(post_approval_state))
        .get_result::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn delete_post(post_id: &Uuid, conn_pool: &Pool) -> Result<usize, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::delete(posts.filter(id.eq(post_id)))
        .execute(&mut conn)
        .map_err(|_| Errors::BadRequest("Failed to delete post as it may not exist".to_string()))
}

pub fn update_title(
    new_title: &str,
    post_id: &Uuid,
    conn_pool: &Pool,
) -> Result<PostModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(title.eq(new_title))
        .get_result::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}
