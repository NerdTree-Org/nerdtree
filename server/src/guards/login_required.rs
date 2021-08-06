use crate::db::user::models::UserModel;
use actix_web::{FromRequest, HttpRequest};
use crate::errors::Errors;
use futures_util::future::{Ready, err, ok};
use actix_web::dev::Payload;
use crate::jwt::verify::verify_token;
use crate::jwt::claims::TokenType;
use actix_web::web::Data;
use crate::db::Pool;
use crate::db::user::query::get_user_by_id;

pub struct LoginRequired {
    pub user: UserModel
}

impl FromRequest for LoginRequired {
    type Error = Errors;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let header_bearer_token = match req.headers().get("authorization") {
            Some(t) => t,
            None => return err(Errors::AccessForbidden)
        }.to_str().unwrap();

        if header_bearer_token.is_empty() {
            return err(Errors::AccessForbidden);
        }

        let splitted_bearer = header_bearer_token.split_ascii_whitespace().collect::<Vec<&str>>();
        if splitted_bearer.len() != 2 {
            return err(Errors::AccessForbidden);
        }

        let raw_token = splitted_bearer[1].clone();
        let token = match verify_token(&raw_token) {
            Ok(t) => t,
            Err(_) => return err(Errors::AccessForbidden)
        };

        // check if token is accesstoken
        match token.token_type {
            TokenType::AccessToken => (),
            _ => return err(Errors::AccessForbidden)
        }

        let conn_pool = match req.app_data::<Data<Pool>>() {
            Some(c) => c,
            None => return err(Errors::InternalServerError)
        };

        let user = match get_user_by_id(&token.id.parse::<uuid::Uuid>().unwrap(), conn_pool.as_ref()) {
            Ok(u) => u,
            Err(_) => return err(Errors::AccessForbidden)
        };
        if user.len() == 0 {
            return err(Errors::AccessForbidden);
        }

        let user = user[0].clone();

        ok(LoginRequired {
            user
        })
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }

    fn configure<F>(f: F) -> Self::Config where F: FnOnce(Self::Config) -> Self::Config {
        f(())
    }
}