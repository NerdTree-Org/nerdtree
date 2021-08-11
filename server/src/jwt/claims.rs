use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum TokenType {
    AccessToken,
    RefreshToken,
    PasswordResetToken,
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub id: String,
    pub exp: i64,
    pub token_type: TokenType,
}
