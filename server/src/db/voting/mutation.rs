use crate::db::voting::models::{DownvoteModel, NewDownvote, NewUpvote, UpvoteModel};
use crate::db::{get_conn, Pool};
use crate::errors::Errors;
use diesel::prelude::*;
use uuid::Uuid;

pub fn add_upvote(
    v_user_id: Uuid,
    v_post_id: Uuid,
    conn_pool: &Pool,
) -> Result<UpvoteModel, Errors> {
    use crate::db::schema::upvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_upvote = NewUpvote {
        post_id: v_post_id,
        user_id: v_user_id,
    };

    diesel::insert_into(upvotes)
        .values(new_upvote)
        .get_result::<UpvoteModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn add_downvote(
    v_user_id: Uuid,
    v_post_id: Uuid,
    conn_pool: &Pool,
) -> Result<DownvoteModel, Errors> {
    use crate::db::schema::downvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_downvote = NewDownvote {
        post_id: v_post_id,
        user_id: v_user_id,
    };

    diesel::insert_into(downvotes)
        .values(new_downvote)
        .get_result::<DownvoteModel>(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn remove_upvote(v_user_id: Uuid, v_post_id: Uuid, conn_pool: &Pool) -> Result<usize, Errors> {
    use crate::db::schema::upvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::delete(upvotes.filter(post_id.eq(v_post_id).and(user_id.eq(v_user_id))))
        .execute(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}

pub fn remove_downvote(
    v_user_id: Uuid,
    v_post_id: Uuid,
    conn_pool: &Pool,
) -> Result<usize, Errors> {
    use crate::db::schema::downvotes::dsl::*;
    let mut conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::delete(downvotes.filter(post_id.eq(v_post_id).and(user_id.eq(v_user_id))))
        .execute(&mut conn)
        .map_err(|_| Errors::InternalServerError)
}
