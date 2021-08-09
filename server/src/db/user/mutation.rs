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

pub fn update_user_password(
    user_id: &uuid::Uuid,
    new_password: &str,
    conn_pool: &Pool
) -> Result<UserModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(password.eq(new_password))
        .get_result::<UserModel>(&conn)
        .map_or_else(|_| Err(Errors::InternalServerError), Ok)
}

pub fn update_user_firstname(
    user_id: &uuid::Uuid,
    new_firstname: &str,
    conn_pool: &Pool
) -> Result<UserModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(firstname.eq(new_firstname))
        .get_result::<UserModel>(&conn)
        .map_or_else(|_| Err(Errors::InternalServerError), Ok)
}

pub fn update_user_lastname(
    user_id: &uuid::Uuid,
    new_lastname: &str,
    conn_pool: &Pool
) -> Result<UserModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(lastname.eq(new_lastname))
        .get_result::<UserModel>(&conn)
        .map_or_else(|_| Err(Errors::InternalServerError), Ok)
}

pub fn update_user_email(
    user_id: &uuid::Uuid,
    new_email: &str,
    conn_pool: &Pool
) -> Result<UserModel, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::update(users.filter(id.eq(user_id)))
        .set(email.eq(new_email))
        .get_result::<UserModel>(&conn)
        .map_or_else(|_| Err(Errors::InternalServerError), Ok)
}

pub fn delete_user(
    user_id: &uuid::Uuid,
    conn_pool: &Pool
) -> Result<usize, Errors> {
    let conn = get_conn(conn_pool).map_err(|_| Errors::InternalServerError)?;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(&conn)
        .map_err(|_| Errors::BadRequest("Failed to delete user as it may not exist"))
}