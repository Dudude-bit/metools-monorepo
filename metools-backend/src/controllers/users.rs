use crate::controllers::users::UsersError::InvalidInputData;
use crate::models::users::User;
use crate::services::users::UsersServiceError;
use crate::AppState;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, http, post, web, App, HttpResponse, Responder, ResponseError};
use derive_more::{Display, Error};
use serde::de::Unexpected::Str;
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
    UnknownError,
}

impl From<UsersServiceError> for UsersError {
    fn from(value: UsersServiceError) -> Self {
        return match value {
            UsersServiceError::UnknownError => {}
        };
    }
}

impl ResponseError for UsersError {
    fn status_code(&self) -> StatusCode {
        return match self {
            InvalidInputData(_) => StatusCode::BAD_REQUEST,
            UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
        };
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        return match self {
            InvalidInputData(errors) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(
                    json!({"message": "Invalid input data", "status": "invalid_data"}).to_string(),
                ),
            UnknownError => HttpResponse::build(self.status_code())
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
    return match data.validate() {
        Ok(_) => {
            web::block(move || {
                let r = state.users_service.register_user(
                    data.username.clone(),
                    data.email.clone(),
                    data.password.clone(),
                );
                match r {
                    Ok(user) => return Js,
                    Err(err) => {}
                }
            })
            .await
            .unwrap();
            Ok(String::from("123"))
        }
        Err(err) => Err(InvalidInputData(err)),
    };
}

#[post("/login")]
async fn login(
    data: web::Json<LoginData>,
    state: web::Data<AppState>,
) -> Result<impl Responder, UsersError> {
    return match data.validate() {
        Ok(_) => Ok(String::from("123")),
        Err(err) => Err(InvalidInputData(err)),
    };
}
