use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
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
        let service_url = env::var("SERVICE_URL").unwrap_or(http_address.clone());
        let db_url = env::var("DATABASE_URL").unwrap_or(String::from(
            "postgresql://postgres:postgres@localhost:5432/metools",
        ));
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set"); // In minutes
        let run_migrations = env::var("RUN_MIGRATIONS").unwrap_or(String::from("false"));
        let smtp_from = env::var("SMTP_FROM").expect("SMTP_FROM must be set");
        let smtp_hostname = env::var("SMTP_HOSTNAME").expect("SMTP_HOSTNAME must be set");
        let smtp_port = env::var("SMTP_PORT").unwrap_or(String::from("587")); // Default port is 587
        let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
        let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");

        Self {
            db_url,
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
