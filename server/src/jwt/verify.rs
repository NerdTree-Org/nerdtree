use super::claims::Token;
use jsonwebtoken::{
    decode, errors::Error, errors::Result, Algorithm, DecodingKey, TokenData, Validation,
};
use std::env::var;

type TokenReturnType = std::result::Result<Token, Error>;

fn __internal_verify(token: &str) -> Result<TokenData<Token>> {
    decode::<Token>(
        token,
        &DecodingKey::from_secret(var("JWT_SECRET_KEY").unwrap().as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
}

fn __internal_verify_with_custom_secret(token: &str, secret: &str) -> Result<TokenData<Token>> {
    decode::<Token>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
}

pub fn verify_token(token: &str) -> TokenReturnType {
    match __internal_verify(token) {
        Ok(cl) => Ok(cl.claims),
        Err(e) => Err(e),
    }
}

pub fn verify_token_with_custom_secret(token: &str, secret: &str) -> TokenReturnType {
    match __internal_verify_with_custom_secret(token, secret) {
        Ok(cl) => Ok(cl.claims),
        Err(e) => Err(e),
    }
}
