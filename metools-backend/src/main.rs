use std::collections::HashMap;
use std::env;

use crate::models::tasks::{list_all_tasks, list_all_users_tasks};
use actix_web::{web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use utoipa::OpenApi;
use utoipa::openapi::{Info, OpenApiBuilder};
use uuid::Uuid;

mod controllers;
mod models;
mod schema;
mod utils;

type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;
#[derive(OpenApi)]
#[openapi(
    info(description = "Documentation to MeTools API", title = "MeTools", )
)]
struct OpenAPI;

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

    let doc = OpenAPI::openapi();
    println!("{:?}", list_all_users_tasks(&mut pool.get().unwrap(), "779c9940-654e-4b34-85cc-49b3cf60273f".parse().unwrap()));
}
