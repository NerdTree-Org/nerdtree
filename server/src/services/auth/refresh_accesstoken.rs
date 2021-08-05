use actix_web::{ web::Json, Responder};
use crate::services::auth::payloads::{RefreshAccessTokenPayload, RefreshAccessTokenReturnPayload};
use crate::errors::Errors;
use crate::jwt::verify::verify_token;
use crate::jwt::claims::TokenType;
use crate::jwt::sign::generate_accesstoken;

pub async fn refresh_accesstoken(payload: Json<RefreshAccessTokenPayload>) -> Result<impl Responder, Errors> {
    let refresh_token = payload.refresh_token.clone();
    let token_payload = verify_token(&refresh_token).map_err(|_| Errors::BadRequest("Invalid token"))?;

    match token_payload.token_type {
        TokenType::RefreshToken => (),
        _ => return Err(Errors::BadRequest("Invalid token"))
    }

    let new_accesstoken = generate_accesstoken(token_payload.id.parse().unwrap()).map_err(|_| Errors::InternalServerError)?;

    Ok(Json(RefreshAccessTokenReturnPayload {
        access_token: new_accesstoken
    }))
}