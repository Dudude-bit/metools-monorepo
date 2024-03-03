use crate::services::users::UsersServiceError;
use crate::AppState;
use actix_web::body::BoxBody;

use actix_web::cookie::{time::Duration as ActixWebDuration, Cookie};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, post, web, HttpResponse, Responder, ResponseError};
use chrono::{Duration, Utc};
use derive_more::Display;
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::controllers::middlewares::UserMiddleware;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
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
                .body(json!({"error": "Invalid input data", "status": "invalid_data"}).to_string()),
            Self::UsersServiceError(_service_err) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
            Self::UnknownError => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
        };
    }
}

#[get("/me")]
async fn me(user: UserMiddleware) -> Result<impl Responder, UsersError> {
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
                Ok(user) => Ok(web::Json(json!({"status": "success", "data": user}))),
                Err(err) => Err(UsersError::UsersServiceError(err)),
            }
        }
        Err(err) => Err(UsersError::InvalidInputData(err)),
    }
}

#[post("/login")]
async fn login(
    data: web::Json<LoginData>,
    state: web::Data<AppState>,
) -> Result<impl Responder, UsersError> {
    match data.validate() {
        Ok(_) => {
            let r = state
                .users_service
                .authenticate_user(data.username.clone(), data.password.clone());
            match r {
                Ok(user) => {
                    let now = Utc::now();
                    let iat = now.timestamp() as usize;
                    let exp = (now + Duration::minutes(60)).timestamp() as usize;
                    let claims: TokenClaims = TokenClaims {
                        sub: user.id.to_string(),
                        exp,
                        iat,
                    };

                    let token = encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
                    )
                    .unwrap();

                    let cookie = Cookie::build("token", token.to_owned())
                        .path("/")
                        .max_age(ActixWebDuration::new(60 * 60, 0))
                        .http_only(true)
                        .finish();
                    Ok(HttpResponse::Ok()
                        .cookie(cookie)
                        .json(json!({"status": "success", "data": {"token": token}})))
                }
                Err(err) => Err(UsersError::UsersServiceError(err)),
            }
        }
        Err(err) => Err(UsersError::InvalidInputData(err)),
    }
}

#[get("/logout")]
async fn logout_handler(_: UserMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .json(json!({"status": "success"}))
}
