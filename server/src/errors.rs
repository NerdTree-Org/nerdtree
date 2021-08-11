use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};

use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display)]
pub enum Errors {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(String),

    #[display(fmt = "Access Forbidden")]
    AccessForbidden,
}

#[derive(Serialize)]
struct ReturnPayload {
    pub success: bool,
    pub error: String,
}

impl ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match *self {
            Errors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::BadRequest(_) => StatusCode::BAD_REQUEST,
            Errors::AccessForbidden => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Errors::InternalServerError => {
                HttpResponse::InternalServerError().json(ReturnPayload {
                    success: false,
                    error: String::from("Internal Server Error"),
                })
            }
            Errors::BadRequest(ref message) => HttpResponse::BadRequest().json(ReturnPayload {
                success: false,
                error: message.clone(),
            }),
            Errors::AccessForbidden => HttpResponse::Forbidden().json(ReturnPayload {
                success: false,
                error: String::from("Access forbidden"),
            }),
        }
    }
}
