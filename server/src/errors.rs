use actix_web::{
    error::ResponseError,
    HttpResponse,
    http::StatusCode
};

use serde::Serialize;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum Errors {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request: {}", _0)]
    BadRequest(&'static str),

    #[display(fmt = "Access Forbidden")]
    AccessForbidden
}

#[derive(Serialize)]
struct ReturnPayload {
    pub success: bool,
    pub error: &'static str
}

impl ResponseError for Errors {
    fn status_code(&self) -> StatusCode {
        match *self {
            Errors::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Errors::BadRequest(_) => StatusCode::BAD_REQUEST,
            Errors::AccessForbidden => StatusCode::FORBIDDEN
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            Errors::InternalServerError => {
                HttpResponse::InternalServerError().json(ReturnPayload {
                    success: false,
                    error: "Internal Server Error"
                })
            },
            Errors::BadRequest(message) => HttpResponse::BadRequest().json(ReturnPayload {
                success: false,
                error: message
            }),
            Errors::AccessForbidden => HttpResponse::Forbidden().json(ReturnPayload {
                success: false,
                error: "Access forbidden"
            })
        }
    }
}