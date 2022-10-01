use crate::db::post::models::PostModel;
use crate::db::schema::posts::dsl::*;
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_post_by_uuid(post_uuid: Uuid, conn_pool: &Pool) -> Result<Vec<PostModel>, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    posts
        .filter(id.eq(post_uuid))
        .limit(1)
        .load::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_all_posts(conn_pool: &Pool) -> Result<Vec<PostModel>, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    posts
        .load::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_posts_by_author_id(
    author_uuid: Uuid,
    conn_pool: &Pool,
) -> Result<Vec<PostModel>, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    posts
        .filter(post_author.eq(author_uuid))
        .load::<PostModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}
