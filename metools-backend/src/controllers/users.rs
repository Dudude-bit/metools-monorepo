use actix_web::{get, post, Responder, web};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct SignUpData {
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 512))]
    password: String,
    #[validate(must_match = "password")]
    repeat_password: String
}

#[derive(Deserialize, Validate)]
struct LoginData {
    username: String,
    #[validate(length(min = 8, max = 512))]
    password: String
}

#[get("/me")]
async fn me() -> impl Responder {
    String::from("me")
}

#[post("/signup")]
async fn signup(data: web::Json<SignUpData>) -> impl Responder {
    String::from("signup")
}

#[post("/login")]
async fn login(data: web::Json<LoginData>) -> impl Responder {
    String::from("login")
}
