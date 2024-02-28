use std::collections::HashMap;
use std::env;

use crate::controllers::users::{login, me, signup};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use log::log;
use utoipa::OpenApi;

mod controllers;
mod models;
mod schema;
mod utils;

type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
#[derive(OpenApi)]
#[openapi(info(description = "Documentation to MeTools API", title = "MeTools"))]
struct OpenAPI;

async fn swagger() -> impl Responder {
    web::Json(OpenAPI::openapi())
}

#[actix_web::main]
async fn main() {
    let http_address = env::var("HTTP_ADDRESS").unwrap_or(String::from("127.0.0.1:8000"));
    let db_url = env::var("DB_URL").unwrap_or(String::from(
        "postgresql://postgres:postgres@localhost:5432/metools",
    ));
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: DBPool = r2d2::Pool::builder()
        .build(manager)
        .expect("failed to create pg pool");

    HttpServer::new(|| {
        App::new()
            .route("/swagger", web::get().to(swagger))
            .service(
                web::scope("/api/v1")
                    .service(me)
                    .service(login)
                    .service(signup),
            )
    })
    .bind(http_address.clone())
    .expect(format!("failed to bind to {}", http_address).as_str())
    .run()
    .await
    .expect("failed to run server");
}
