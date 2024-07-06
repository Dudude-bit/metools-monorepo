use std::env;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

#[derive(Debug, Clone)]
pub struct DBConfig {
    pub surrealdb_url: String,
    pub surrealdb_username: String,
    pub surrealdb_password: String,
    pub surrealdb_ns: String,
    pub surrealdb_db: String,
}

impl DBConfig {
    pub async fn get_connection(&self) -> Surreal<Client> {
        let db = Surreal::new::<Ws>(self.surrealdb_url.as_str())
            .await
            .unwrap();

        db.signin(Root {
            username: self.surrealdb_username.as_str(),
            password: self.surrealdb_password.as_str(),
        })
        .await
        .unwrap();
        db.use_ns(self.surrealdb_ns.as_str())
            .use_db(self.surrealdb_password.as_str())
            .await
            .unwrap();

        db
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db: DBConfig,
    pub service_url: String,
    pub http_address: String,
    pub jwt_secret: String,
    pub jwt_maxage: usize,
    pub run_migrations: bool,
    pub smtp_from: String,
    pub smtp_hostname: String,
    pub smtp_port: usize,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl Config {
    pub fn init() -> Self {
        let http_address = env::var("HTTP_ADDRESS").unwrap_or(String::from("0.0.0.0:8000"));
        let service_url =
            env::var("SERVICE_URL").unwrap_or(format!("http://{}", http_address.clone()));
        let surrealdb_url =
            env::var("SURREALDB_URL").unwrap_or(String::from("ws://localhost:8000"));
        let surrealdb_username = env::var("SURREALDB_USERNAME").unwrap_or(String::from("root"));
        let surrealdb_password = env::var("SURREALDB_PASSWORD").unwrap_or(String::from("root"));
        let surrealdb_ns = env::var("SURREALDB_NS").unwrap_or(String::from("ns"));
        let surrealdb_db = env::var("SURREALDB_DB").unwrap_or(String::from("db"));
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set"); // In minutes
        let run_migrations = env::var("RUN_MIGRATIONS").unwrap_or(String::from("false"));
        let smtp_from = env::var("SMTP_FROM").expect("SMTP_FROM must be set");
        let smtp_hostname = env::var("SMTP_HOSTNAME").expect("SMTP_HOSTNAME must be set");
        let smtp_port = env::var("SMTP_PORT").unwrap_or(String::from("587")); // Default port is 587
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        Self {
            db: DBConfig {
                surrealdb_url,
                surrealdb_username,
                surrealdb_password,
                surrealdb_ns,
                surrealdb_db,
            },
            http_address,
            service_url,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<usize>().unwrap(),
            run_migrations: run_migrations.parse::<bool>().unwrap(),
            smtp_from,
            smtp_hostname,
            smtp_port: smtp_port.parse::<usize>().unwrap(),
            smtp_username,
            smtp_password,
        }
    }
}
