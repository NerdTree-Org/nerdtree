#[macro_use]
extern crate diesel;
use actix_web::{HttpServer, App, web};
use crate::db::create_db_pool;

mod errors;
mod services;
mod db;
mod jwt;

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
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}