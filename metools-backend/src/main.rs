mod config;
mod controllers;
mod models;
mod schema;
mod utils;
mod services;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use env_logger;
use utoipa::OpenApi;

use crate::controllers::users::{login, me, signup};
use config::Config;

type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
#[derive(OpenApi)]
#[openapi(info(description = "Documentation to MeTools API", title = "MeTools"))]
struct OpenAPI;

async fn swagger() -> impl Responder {
    web::Json(OpenAPI::openapi())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = Config::init();
    let manager = ConnectionManager::<PgConnection>::new(config.db_url);
    let pool: DBPool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pg pool");

    HttpServer::new(|| {
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
    })
    .bind(config.http_address.clone())
    .expect(format!("failed to bind to {}", config.http_address).as_str())
    .run()
    .await
}
