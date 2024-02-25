use actix_web::{get, Responder};

#[get("/me")]
fn me() -> impl Responder {

}