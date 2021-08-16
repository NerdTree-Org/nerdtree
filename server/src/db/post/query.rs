use uuid::Uuid;
use crate::db::{Pool, get_conn};
use crate::db::post::models::PostModel;
use crate::errors::Errors;
use crate::db::schema::posts::dsl::*;
use diesel::prelude::*;

pub fn get_post_by_uuid(
    post_uuid: Uuid,
    conn_pool: &Pool
) -> Result<Vec<PostModel>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    posts
        .filter(id.eq(post_uuid))
        .limit(1)
        .load::<PostModel>(&conn)
        .map_err(|_| Errors::InternalServerError)
}