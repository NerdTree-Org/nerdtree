#[macro_use]
extern crate diesel;
use actix_web::{HttpServer, App, web};
use crate::db::create_db_pool;

pub mod errors;
mod services;
pub mod db;
pub mod jwt;
pub mod env;
pub mod email;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .data(create_db_pool())
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(services::auth::login::login_handler))
                    .route("/register", web::post().to(services::auth::register::register_handler))
                    .route("/refresh_token", web::post().to(services::auth::refresh_accesstoken::refresh_accesstoken))
                    .route("/password_reset/request", web::post().to(services::auth::password_reset::password_reset_request_handler))
                    .route("/password_reset/token", web::post().to(services::auth::password_reset::password_reset_token_handler))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}