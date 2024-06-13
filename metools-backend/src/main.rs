mod config;
mod controllers;
mod models;
mod schema;
mod services;
mod utils;

use std::env;
use std::fs::File;
use std::io::Write;

use actix_cors::Cors;
use actix_web::body::MessageBody;
use actix_web::http::header;
use actix_web::middleware::{Compress, Logger};
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_prometheus::PrometheusMetricsBuilder;
use controllers::rzd::tasks::{
    create_task, delete_all_tasks_for_user, delete_task_by_id_for_user, list_tasks,
};
use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
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
        controllers::rzd::tasks::delete_all_tasks_for_user
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
        crate::controllers::schema::ResponseDeleteAllTasksForUser,
        crate::models::users::UserReturn,
        crate::models::rzd::tasks::Task
    ))
)]
struct OpenAPI;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(conn: &mut PgConnection) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::args().len() > 1 {
        match env::args().nth(1).unwrap().as_str() {
            "gen-swagger" => {
                let path = env::args().nth(2).expect("Need path parameter");
                let mut file = File::create(path.clone())
                    .unwrap_or_else(|_| panic!("Cant create file with path {}", path.clone()));
                let yaml_string = serde_yaml::to_string(&OpenAPI::openapi()).unwrap();
                file.write_all(&yaml_string.try_into_bytes().unwrap())
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
        if config.run_migrations {
            log::info!("Running migrations");
            run_migrations(&mut pool.get().unwrap());
            log::info!("Ran migrations");
        }
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .expose_any_header()
            .max_age(3600);
        let prometheus = PrometheusMetricsBuilder::new("api")
            .endpoint("/metrics")
            .build()
            .unwrap();
        App::new()
            .service(me)
            .service(login)
            .service(signup)
            .service(list_tasks)
            .service(create_task)
            .service(delete_task_by_id_for_user)
            .service(delete_all_tasks_for_user)
            .service(
                SwaggerUi::new("/swagger/{_:.*}").url("/openapi.json", OpenAPI::openapi().clone()),
            )
            .service(web::resource("/healthz").to(health))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(prometheus)
            .wrap(cors)
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
