use crate::db::post::models::{NewPost, PostModel};
use crate::db::schema::posts::dsl::*;
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::RunQueryDsl;

pub fn insert_post(
    post_title: &str,
    post_body: &str,
    post_creator: &uuid::Uuid,
    conn_pool: &Pool,
) -> Result<PostModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_post = NewPost {
        title: post_title,
        body: post_body,
        post_author: post_creator,
    };

    diesel::insert_into(posts)
        .values(new_post)
        .get_result::<PostModel>(&conn)
        .map_err(|_| Errors::InternalServerError)
}
