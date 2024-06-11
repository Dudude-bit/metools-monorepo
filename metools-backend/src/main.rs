mod config;
mod controllers;
mod models;
mod schema;
mod services;
mod utils;

use std::env;
use std::fs::File;
use std::io::Write;

use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpServer};
use controllers::rzd::tasks::{create_task, delete_task_by_id_for_user, list_tasks};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use services::tasks::TasksService;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::config::Config;
use crate::controllers::schema::AppState;
use crate::controllers::users::users::{login, me, signup};
use crate::models::DBPool;
use crate::services::users::UsersService;

#[derive(OpenApi)]
#[openapi(
    info(description = "Documentation to MeTools API", title = "MeTools"),
    paths(
        controllers::users::users::me,
        controllers::users::users::login,
        controllers::users::users::signup,
        controllers::rzd::tasks::list_tasks,
        controllers::rzd::tasks::create_task,
        controllers::rzd::tasks::delete_task_by_id_for_user,
    ),
    components(schemas(
        crate::controllers::users::users::LoginData,
        crate::controllers::users::users::SignUpData,
        crate::controllers::rzd::tasks::CreateTaskData,
        crate::controllers::schema::ErrorResponse,
        crate::controllers::schema::ResponseMe,
        crate::controllers::schema::ResponseListTasks,
        crate::controllers::schema::ResponseCreateTask,
        crate::controllers::schema::ResponseDeleteTaskByIdForUser,
        crate::models::users::UserReturn,
        crate::models::rzd::tasks::Task
    ))
)]
struct OpenAPI;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::args().len() > 1 {
        match env::args().nth(1).unwrap().as_str() {
            "gen-swagger" => {
                let path = env::args().nth(2).expect("Need path parameter");
                let mut file = File::create(path.clone())
                    .unwrap_or_else(|_| panic!("Cant create file with path {}", path.clone()));
                file.write_all(OpenAPI::openapi().to_pretty_json().unwrap().as_bytes())
                    .unwrap_or_else(|_| panic!("Cant write to file with path {}", path.clone()))
            }
            &_ => {
                panic!("Unknown CLI command");
            }
        }
        return Ok(());
    }

    env::set_var("RUST_LOG", "debug");

    env_logger::init();
    let config = Config::init();

    HttpServer::new(move || {
        let manager = ConnectionManager::<PgConnection>::new(config.db_url.clone());
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("failed to create pg pool");
        App::new()
            .service(me)
            .service(login)
            .service(signup)
            .service(list_tasks)
            .service(create_task)
            .service(delete_task_by_id_for_user)
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/openapi.json", OpenAPI::openapi().clone()),
            )
            .wrap(Logger::default())
            .wrap(Compress::default())
            .app_data(web::Data::new(AppState {
                users_service: UsersService::init(pool.clone()),
                tasks_service: TasksService::init(pool.clone()),
                jwt_secret: config.jwt_secret.clone(),
                jwt_maxage: config.jwt_maxage,
            }))
    })
    .bind(config.http_address.clone())
    .unwrap_or_else(|_| panic!("failed to bind to {}", config.http_address))
    .run()
    .await
}
