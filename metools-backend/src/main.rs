use std::{env};
use std::collections::HashMap;

use actix_web::{App, HttpServer, web};
use diesel::{PgConnection, r2d2};
use diesel::r2d2::ConnectionManager;

mod models;
mod controllers;
mod schema;
mod utils;

type DBPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() {
    let http_address = env::var("HTTP_ADDRESS").unwrap_or(String::from("127.0.0.1:8000"));
    let db_url = env::var("DB_URL").unwrap_or(String::from("postgresql://postgres:postgres@localhost:5432/metools"));
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: DBPool = r2d2::Pool::builder().build(manager).expect("failed to create pg pool");
    models::tasks::insert_new_task(&mut pool.get().expect("failed"), String::from("day"), HashMap::new());
}
