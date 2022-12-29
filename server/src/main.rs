#[macro_use]
extern crate diesel;
use crate::db::create_db_pool;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http, web, App, HttpServer};

pub mod db;
pub mod email;
pub mod env;
pub mod errors;
pub mod guards;
pub mod jwt;
pub mod paginated_vec;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env::check_env();
    env_logger::init();

    HttpServer::new(|| {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .max_age(None);

        cors = cors.allowed_origin(&std::env::var("WEBSITE_LINK").unwrap());

        let json_config = web::JsonConfig::default()
            .limit(1024 * 1024 * 10);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(Data::new(create_db_pool()))
            .app_data(json_config)
            .service(
                web::scope("/auth")
                    .route(
                        "/login",
                        web::post().to(services::auth::login::login_handler),
                    )
                    .route(
                        "/register",
                        web::post().to(services::auth::register::register_handler),
                    )
                    .route(
                        "/refresh_token",
                        web::post().to(services::auth::refresh_accesstoken::refresh_accesstoken),
                    )
                    .route(
                        "/password_reset/request",
                        web::post()
                            .to(services::auth::password_reset::password_reset_request_handler),
                    )
                    .route(
                        "/password_reset/token",
                        web::post()
                            .to(services::auth::password_reset::password_reset_token_handler),
                    ),
            )
            .service(
                web::scope("/user")
                    .service(
                        web::scope("/update")
                            .route(
                                "/firstname",
                                web::post()
                                    .to(services::user::data_update::update_firstname_handler),
                            )
                            .route(
                                "/lastname",
                                web::post()
                                    .to(services::user::data_update::update_lastname_handler),
                            )
                            .route(
                                "/email",
                                web::post().to(services::user::data_update::update_email_handler),
                            )
                            .route(
                                "/password",
                                web::post()
                                    .to(services::user::data_update::update_password_handler),
                            )
                            .route(
                                "/profile_pic",
                                web::post()
                                    .to(services::user::data_update::upload_profile_pic_handler),
                            ),
                    )
                    .service(
                        web::scope("/query")
                            .route(
                                "/id",
                                web::post().to(services::user::data_query::get_user_by_id_handler),
                            )
                            .route(
                                "/username",
                                web::post()
                                    .to(services::user::data_query::get_user_by_username_handler),
                            )
                            .route(
                                "/email",
                                web::post()
                                    .to(services::user::data_query::get_user_by_email_handler),
                            )
                            .route(
                                "/current",
                                web::post()
                                    .to(services::user::data_query::get_current_user_handler),
                            ),
                    )
                    .service(web::scope("/get").route(
                        "/discord_token",
                        web::post().to(services::user::data_query::get_discord_token),
                    ))
                    .service(web::scope("/delete").route(
                        "/user",
                        web::post().to(services::user::data_update::delete_user_handler),
                    )),
            )
            .service(
                web::scope("/post")
                    .service(
                        web::scope("/update")
                            .route(
                                "/new",
                                web::post().to(services::blog::data_update::new_post_handler),
                            )
                            .route(
                                "/thumbnail",
                                web::post()
                                    .to(services::blog::data_update::upload_thumbnail_thumbnail),
                            )
                            .route(
                                "/body",
                                web::post()
                                    .to(services::blog::data_update::update_post_body_handler),
                            )
                            .route(
                                "/update_approval",
                                web::post().to(services::blog::data_update::approve_post_handler),
                            )
                            .route(
                                "/delete",
                                web::post().to(services::blog::data_update::delete_post_handler),
                            )
                            .route(
                                "/title",
                                web::post()
                                    .to(services::blog::data_update::update_post_title_handler),
                            ),
                    )
                    .service(
                        web::scope("/query")
                            .service(
                                web::scope("/unapproved")
                                    .route(
                                        "/paginate",
                                        web::post().to(services::blog::data_query::paginate_posts_handler_including_unapproved)
                                    )
                                    .route(
                                        "/id",
                                        web::post().to(services::blog::data_query::get_post_by_id_handler_including_unapproved),
                                    )
                                    .route(
                                        "/author_id",
                                        web::post().to(services::blog::data_query::get_posts_by_author_id_handler_including_unapproved)
                                    )
                            )
                            .route(
                                "/paginate",
                                web::post().to(services::blog::data_query::paginate_posts_handler),
                            )
                            .route(
                                "/id",
                                web::post().to(services::blog::data_query::get_post_by_id_handler),
                            )
                            .route(
                                "/author_id",
                                web::post()
                                    .to(services::blog::data_query::get_posts_by_author_id_handler),
                            ),
                    )
                    .service(
                        web::scope("/vote")
                            .route(
                                "/upvote",
                                web::post().to(services::blog::voting::add_upvote_handler),
                            )
                            .route(
                                "/downvote",
                                web::post().to(services::blog::voting::add_downvote_handler),
                            )
                            .route("/votes", web::post().to(services::blog::voting::get_votes))
                            .route(
                                "/by_current_user",
                                web::post().to(services::blog::voting::get_user_vote_for_post),
                            ),
                    ),
            )
            .service(
                web::scope("/discord")
                    .route("/verify", web::post().to(services::discord::verify_token)),
            )
            .service(
                web::scope("/comment")
                    .service(
                        web::scope("/update")
                            .route(
                                "/new",
                                web::post().to(services::comment::data_update::new_comment_handler),
                            )
                            .route(
                                "/edit",
                                web::post()
                                    .to(services::comment::data_update::edit_comment_handler),
                            )
                            .route(
                                "/delete",
                                web::post()
                                    .to(services::comment::data_update::delete_comment_handler),
                            ),
                    )
                    .service(
                        web::scope("/query")
                            .route(
                                "/by_post",
                                web::post().to(
                                    services::comment::data_query::get_comments_by_post_handler,
                                ),
                            )
                            .route(
                                "/by_user",
                                web::post().to(
                                    services::comment::data_query::get_comments_by_user_handler,
                                ),
                            ),
                    ),
            )
            .service(
                actix_files::Files::new("/static", &std::env::var("IMAGE_PATH").unwrap())
                    .show_files_listing(),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
