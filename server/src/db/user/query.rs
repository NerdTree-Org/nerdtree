use crate::db::user::models::UserModel;
use crate::errors::Errors;
use crate::db::{Pool, get_conn};
use diesel::prelude::*;
use crate::db::schema::users::dsl::*;

pub fn get_users_by_email(
    user_email: &str,
    conn_pool: &Pool
) -> Result<Vec<UserModel>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let results = users.filter(email.like(user_email))
        .load::<UserModel>(&conn)
        .map_err(|_| Errors::InternalServerError)?;

    Ok(results)
}

pub fn get_users_by_username(
    user_username: &str,
    conn_pool: &Pool
) -> Result<Vec<UserModel>, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let results = users.filter(username.like(user_username))
        .load::<UserModel>(&conn)
        .map_err(|_| Errors::InternalServerError)?;

    Ok(results)
}

pub fn is_unique_user(
    user_email: &str,
    user_username: &str,
    conn_pool: &Pool
) -> Result<bool, Errors> {
    let usercount_email = get_users_by_email(user_email, conn_pool)?.len();
    let usercount_username = get_users_by_username(user_username, conn_pool)?.len();

    if usercount_email == 0 && usercount_username == 0 {
        Ok(true)
    }
    else {
        Ok(false)
    }
}