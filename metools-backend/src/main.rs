mod config;
mod controllers;
mod models;
mod schema;
mod services;
mod utils;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use std::env;

use utoipa::OpenApi;

use crate::controllers::users::{login, me, signup};
use crate::models::DBPool;
use crate::services::users::UsersService;
use config::Config;

struct AppState {
    pub users_service: UsersService,
    pub jwt_secret: String,
}

#[derive(OpenApi)]
#[openapi(info(description = "Documentation to MeTools API", title = "MeTools"))]
struct OpenAPI;

async fn swagger() -> impl Responder {
    web::Json(OpenAPI::openapi())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "debug");

    env_logger::init();
    let config = Config::init();

    HttpServer::new(move || {
        let manager = ConnectionManager::<PgConnection>::new(config.db_url.clone());
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pg pool");
        App::new()
            .route("/swagger", web::get().to(swagger))
            .service(
                web::scope("/api/v1").service(
                    web::scope("/users")
                        .service(me)
                        .service(login)
                        .service(signup),
                ),
            )
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                users_service: UsersService::init(pool),
                jwt_secret: config.jwt_secret.clone(),
            }))
    })
    .bind(config.http_address.clone())
    .unwrap_or_else(|_| panic!("failed to bind to {}", config.http_address))
    .run()
    .await
}
