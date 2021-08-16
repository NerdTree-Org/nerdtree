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

pub fn update_thumbnail(
    post_thumbnail: &str,
    post_id: &Uuid,
    conn_pool: &Pool
) -> Result<PostModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(posts.filter(id.eq(post_id)))
        .set(thumbnail.eq(post_thumbnail))
        .get_result::<PostModel>(&conn)
        .map_err(|_| Errors::InternalServerError)
}