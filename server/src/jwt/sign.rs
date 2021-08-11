use super::claims::{Token, TokenType};
use jsonwebtoken::{encode, errors::Result, EncodingKey, Header};

fn __internal_sign(claims: impl serde::Serialize) -> Result<String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("JWT_SECRET_KEY").unwrap().as_bytes()),
    )
}

fn __internal_sign_with_custom_secret(
    claims: impl serde::Serialize,
    secret: &str,
) -> Result<String> {
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn generate_accesstoken(user_id: uuid::Uuid) -> Result<String> {
    let claims = Token {
        id: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
        token_type: TokenType::AccessToken,
    };

    __internal_sign(claims)
}

pub fn generate_passwordresettoken(user_id: uuid::Uuid, secret: &str) -> Result<String> {
    let claims = Token {
        id: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::minutes(5)).timestamp(),
        token_type: TokenType::PasswordResetToken,
    };

    __internal_sign_with_custom_secret(claims, secret)
}

pub fn generate_refreshtoken(user_id: uuid::Uuid) -> Result<String> {
    let claims = Token {
        id: user_id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::days(30)).timestamp(),
        token_type: TokenType::RefreshToken,
    };

    __internal_sign(claims)
}
