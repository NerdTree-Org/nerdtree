use std::env::var;

/// Checks for required environment variables
/// Panics when it's not present
pub fn check_env() {
    var("JWT_SECRET_KEY").unwrap();
    var("DATABASE_URL").unwrap();
    var("SMTP_EMAIL").unwrap();
    var("SMTP_PASSWORD").unwrap();
    var("SMTP_SERVER").unwrap();
    var("IMAGE_PATH").unwrap();
}