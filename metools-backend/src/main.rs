mod config;
mod controllers;
mod models;
mod services;
mod utils;

use std::{env, fs::File, io::Write, path::Path, time::Duration};

use actix_cors::Cors;
use actix_web::{
    body::MessageBody,
    middleware::{Compress, Logger},
    web, App, HttpResponse, HttpServer,
};
use actix_web_prometheus::PrometheusMetricsBuilder;
use controllers::rzd::tasks::{
    create_task, delete_all_tasks_for_user, delete_task_by_id_for_user, list_tasks,
};
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};
use models::verify_tokens::delete_expired_verify_tokens;
use services::{mailer::MailerService, tasks::TasksService};
use surrealdb::{engine::any::connect, opt::auth::Root};
use surrealdb_migrations::MigrationRunner;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::Config,
    controllers::{
        schema::AppState,
        users::users::{login, me, signup, verify_user},
    },
    services::users::UsersService,
};

#[derive(OpenApi)]
#[openapi(
    info(description = "Documentation to MeTools API", title = "MeTools"),
    paths(
        controllers::users::users::me,
        controllers::users::users::login,
        controllers::users::users::signup,
        controllers::users::users::verify_user,
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
    ))
)]
struct OpenAPI;

async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn run_migrations(config: &Config) {
    let db = connect(config.db.surrealdb_url.clone())
        .await
        .expect("cant connect to surrealdb");
    db.signin(Root {
        username: config.db.surrealdb_username.as_str(),
        password: config.db.surrealdb_password.as_str(),
    })
    .await
    .expect("cant auth in surrealdb");
    db.use_ns(config.db.surrealdb_ns.clone())
        .use_db(config.db.surrealdb_db.clone())
        .await
        .expect("cant use ns and db");

    MigrationRunner::new(&db)
        .up()
        .await
        .expect("cant up migrations");
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

    if Path::new(".env").exists() {
        dotenv::dotenv().ok();
    }

    env_logger::init();
    let config = Config::init();
    let creds = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());

    // Open a remote connection to gmail
    let smtp_transport = SmtpTransport::relay(config.smtp_hostname.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    if config.run_migrations {
        log::info!("Running migrations");
        run_migrations(&config).await;
        log::info!("Ran migrations");
    }
    let clonned_db_config = config.db.clone();
    tokio::spawn(async move {
        loop {
            let connection = clonned_db_config.clone().get_connection().await;
            let r = delete_expired_verify_tokens(connection).await;
            match r {
                Ok(c) => log::info!("Deleted {c} verify tokens"),
                Err(err) => log::error!("Error on loop delete_expired_verify_tokens: {err}"),
            }
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    });
    HttpServer::new(move || {
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
            .service(verify_user)
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
                users_service: UsersService::init(
                    config.db.clone(),
                    MailerService::init(
                        smtp_transport.clone(),
                        config.smtp_from.clone(),
                        config.service_url.clone(),
                    ),
                ),
                tasks_service: TasksService::init(config.db.clone()),
                jwt_secret: config.jwt_secret.clone(),
                jwt_maxage: config.jwt_maxage,
            }))
    })
    .bind(config.http_address.clone())
    .unwrap_or_else(|_| panic!("failed to bind to {}", config.http_address))
    .run()
    .await
}
