use crate::db::voting::models::{DownvoteModel, UpvoteModel};
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::prelude::*;
use uuid::Uuid;

pub fn get_upvote_by_user_for_post(
    v_post_id: Uuid,
    v_user_id: Uuid,
    conn_pool: &Pool,
) -> Result<Vec<UpvoteModel>, Errors> {
    use crate::db::schema::upvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    upvotes
        .filter(post_id.eq(v_post_id).and(user_id.eq(v_user_id)))
        .load::<UpvoteModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_downvote_by_user_for_post(
    v_post_id: Uuid,
    v_user_id: Uuid,
    conn_pool: &Pool,
) -> Result<Vec<DownvoteModel>, Errors> {
    use crate::db::schema::downvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    downvotes
        .filter(post_id.eq(v_post_id).and(user_id.eq(v_user_id)))
        .load::<DownvoteModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_upvotes_of_post(v_post_id: Uuid, conn_pool: &Pool) -> Result<Vec<UpvoteModel>, Errors> {
    use crate::db::schema::upvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    upvotes
        .filter(post_id.eq(v_post_id))
        .load::<UpvoteModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn get_downvotes_of_post(
    v_post_id: Uuid,
    conn_pool: &Pool,
) -> Result<Vec<DownvoteModel>, Errors> {
    use crate::db::schema::downvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    downvotes
        .filter(post_id.eq(v_post_id))
        .load::<DownvoteModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}
