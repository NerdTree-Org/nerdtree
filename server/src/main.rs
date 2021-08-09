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
pub mod guards;

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
            .service(
                web::scope("/user")
                    .service(
                        web::scope("/update")
                            .route("/firstname", web::post().to(services::user::data_update::update_firstname_handler))
                            .route("/lastname", web::post().to(services::user::data_update::update_lastname_handler))
                            .route("/email", web::post().to(services::user::data_update::update_email_handler))
                            .route("/password", web::post().to(services::user::data_update::update_password_handler))
                    )
                    .service(
                        web::scope("/query")
                            .route("/id", web::post().to(services::user::data_query::get_user_by_id_handler))
                            .route("/username", web::post().to(services::user::data_query::get_user_by_username_handler))
                            .route("/email", web::post().to(services::user::data_query::get_user_by_email_handler))
                            .route("/current", web::post().to(services::user::data_query::get_current_user_handler))
                    )
                    .service(
                        web::scope("/get")
                            .route("/discord_token", web::post().to(services::user::data_query::get_discord_token))
                    )
                    .service(
                        web::scope("/delete")
                            .route("/user", web::post().to(services::user::data_update::delete_user_handler))
                    )
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}