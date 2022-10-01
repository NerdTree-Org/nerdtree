use crate::db::comment::models::{CommentModel, NewComment};
use crate::db::schema::comments::dsl::*;
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::prelude::*;
use uuid::Uuid;

pub fn insert_comment(
    comment_post_id: &Uuid,
    comment_author_id: &Uuid,
    comment_body: &str,
    conn_pool: &Pool,
) -> Result<CommentModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_comment = NewComment {
        post_id: comment_post_id,
        author_id: comment_author_id,
        body: comment_body,
    };

    diesel::insert_into(comments)
        .values(new_comment)
        .get_result::<CommentModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn edit_comment(
    comment_id: &Uuid,
    new_body: &str,
    conn_pool: &Pool,
) -> Result<CommentModel, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(comments.filter(id.eq(comment_id)))
        .set(body.eq(new_body))
        .get_result::<CommentModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn delete_comment(comment_id: &Uuid, conn_pool: &Pool) -> Result<usize, Errors> {
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::delete(comments.filter(id.eq(comment_id)))
        .execute(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}
