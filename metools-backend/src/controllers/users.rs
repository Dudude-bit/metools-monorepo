use actix_web::{get, Responder};

#[utoipa::path(
get,
path = "/users/me",
responses(
(status = 200, description = "Return info about current user", body = Pet)
)
)]
#[get("/me")]
fn me() -> impl Responder {

}