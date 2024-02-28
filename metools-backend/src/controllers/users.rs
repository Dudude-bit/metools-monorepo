use actix_web::{get, post, Responder};

#[get("/me")]
async fn me() -> impl Responder {
    return String::from("");
}

#[post("/signup")]
async fn signup() -> impl Responder {
    return String::from("");
}

#[post("/login")]
async fn login() -> impl Responder {
    return String::from("");
}
