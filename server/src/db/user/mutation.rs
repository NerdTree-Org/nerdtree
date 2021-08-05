use crate::db::Pool;
use crate::db::get_conn;
use super::models::NewUser;
use super::super::schema::users::dsl::*;
use diesel::prelude::*;
use crate::db::user::models::UserModel;
use crate::errors::Errors;

pub fn insert_user(
    user_username: &str,
    user_firstname: &str,
    user_lastname: &str,
    user_password_hash: &str,
    user_email: &str,
    user_facebook_id: &str,
    conn_pool: &Pool
) -> Result<UserModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    let new_user = NewUser {
        username: user_username,
        firstname: user_firstname,
        lastname: user_lastname,
        password: user_password_hash,
        email: user_email,
        facebook_id: user_facebook_id
    };

    diesel::insert_into(users)
        .values(&new_user)
        .get_result::<UserModel>(&conn)
        .map_or_else(|_| Err(Errors::InternalServerError), Ok)
}