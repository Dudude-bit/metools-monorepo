use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
    pub http_address: String,
    pub jwt_secret: String,
    pub jwt_maxage: i32,
    pub run_migrations: bool,
}

impl Config {
    pub fn init() -> Self {
        let http_address = env::var("HTTP_ADDRESS").unwrap_or(String::from("0.0.0.0:8000"));
        let db_url = env::var("DATABASE_URL").unwrap_or(String::from(
            "postgresql://postgres:postgres@localhost:5432/metools",
        ));
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_maxage = env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set"); // In minutes
        let run_migrations = env::var("RUN_MIGRATIONS").unwrap_or(String::from("false"));

        Self {
            db_url,
            http_address,
            jwt_secret,
            jwt_maxage: jwt_maxage.parse::<i32>().unwrap(),
            run_migrations: run_migrations.parse::<bool>().unwrap(),
        }
    }
}
