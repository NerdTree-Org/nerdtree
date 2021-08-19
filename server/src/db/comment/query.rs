use crate::db::schema::comments::dsl::*;
use diesel::prelude::*;
use uuid::Uuid;
use crate::db::{Pool, get_conn};
use crate::db::comment::models::CommentModel;
use crate::errors::Errors;

pub fn get_comments_by_post(
    comment_post_id: &Uuid,
    conn_pool: &Pool
) -> Result<Vec<CommentModel>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    comments.filter(post_id.eq(comment_post_id))
        .load::<CommentModel>(&conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_comments_by_author(
    comment_author_id: &Uuid,
    conn_pool: &Pool
) -> Result<Vec<CommentModel>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    comments.filter(author_id.eq(comment_author_id))
        .load::<CommentModel>(&conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_comments_by_uuid(
    comment_id: &Uuid,
    conn_pool: &Pool
) -> Result<Vec<CommentModel>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    comments.filter(id.eq(comment_id))
        .load::<CommentModel>(&conn)
        .map_err(|_| Errors::InternalServerError)
}