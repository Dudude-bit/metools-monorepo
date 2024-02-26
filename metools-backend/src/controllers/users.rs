use actix_web::{get, post, Responder};


#[get("/me")]
fn me() -> impl Responder {
    return String::from("");
}

#[post("/signup")]
fn signup() -> impl Responder {

}

#[post("/login")]
fn login() -> impl Responder {

}