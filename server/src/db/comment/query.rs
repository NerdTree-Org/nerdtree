use crate::db::comment::models::CommentModel;
use crate::db::schema::comments::dsl::*;
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_comments_by_post(
    comment_post_id: &Uuid,
    conn_pool: &Pool,
) -> Result<Vec<CommentModel>, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    comments
        .filter(post_id.eq(comment_post_id))
        .order(creation_date.desc())
        .load::<CommentModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_comments_by_author(
    comment_author_id: &Uuid,
    conn_pool: &Pool,
) -> Result<Vec<CommentModel>, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    comments
        .filter(author_id.eq(comment_author_id))
        .order(creation_date.desc())
        .load::<CommentModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_comments_by_uuid(
    comment_id: &Uuid,
    conn_pool: &Pool,
) -> Result<Vec<CommentModel>, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    comments
        .filter(id.eq(comment_id))
        .load::<CommentModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}
