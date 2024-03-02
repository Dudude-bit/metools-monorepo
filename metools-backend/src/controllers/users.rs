use crate::services::users::UsersServiceError;
use crate::AppState;
use actix_web::body::BoxBody;

use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use derive_more::Display;

use serde::Deserialize;
use serde_json::json;
use validator::{Validate, ValidationErrors};

#[derive(Deserialize, Validate)]
struct SignUpData {
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 512))]
    password: String,
    #[validate(must_match = "password")]
    repeat_password: String,
}

#[derive(Deserialize, Validate)]
struct LoginData {
    username: String,
    #[validate(length(min = 8, max = 512))]
    password: String,
}

#[derive(Debug, Display)]
enum UsersError {
    InvalidInputData(ValidationErrors),
    UsersServiceError(UsersServiceError),
    UnknownError,
}

impl ResponseError for UsersError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidInputData(_) => StatusCode::BAD_REQUEST,
            Self::UsersServiceError(_service_err) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        return match self {
            Self::InvalidInputData(_errors) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(
                    json!({"message": "Invalid input data", "status": "invalid_data"}).to_string(),
                ),
            Self::UsersServiceError(_service_err) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"message": "Unknown error", "status": "unknown_error"}).to_string()),
            Self::UnknownError => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"message": "Unknown error", "status": "unknown_error"}).to_string()),
        };
    }
}

#[get("/me")]
async fn me() -> Result<impl Responder, UsersError> {
    Ok(String::from("123"))
}

#[post("/signup")]
async fn signup(
    data: web::Json<SignUpData>,
    state: web::Data<AppState>,
) -> Result<impl Responder, UsersError> {
    match data.validate() {
        Ok(_) => {
            let r = web::block(move || {
                state.users_service.register_user(
                    data.username.clone(),
                    data.email.clone(),
                    data.password.clone(),
                )
            })
            .await
            .unwrap();
            match r {
                Ok(user) => Ok(web::Json(user)),
                Err(err) => Err(UsersError::UsersServiceError(err)),
            }
        }
        Err(err) => Err(UsersError::InvalidInputData(err)),
    }
}

#[post("/login")]
async fn login(
    data: web::Json<LoginData>,
    _state: web::Data<AppState>,
) -> Result<impl Responder, UsersError> {
    match data.validate() {
        Ok(_) => Ok(String::from("123")),
        Err(err) => Err(UsersError::InvalidInputData(err)),
    }
}
