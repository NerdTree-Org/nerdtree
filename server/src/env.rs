use std::env::var;

/// Checks for required environment variables
/// Panics when it's not present
pub fn check_env() {
    var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY is not found in the env");
    var("DATABASE_URL").expect("DATABASE_URL is not found in the env");
    var("SMTP_EMAIL").expect("SMTP_EMAIL is not found in the env");
    var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not found in the env");
    var("SMTP_SERVER").expect("SMTP_SERVER is not found in the env");
    var("IMAGE_PATH").expect("IMAGE_PATH is not found in the env");
    var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKEN is not found in the env");
    var("WEBSITE_LINK").expect("WEBSITE_LINK is not found in the env");
}
