use actix_web::{
    body::BoxBody,
    get,
    http::{header::ContentType, StatusCode},
    post, web, HttpResponse, ResponseError,
};
use chrono::{Duration, Utc};
use derive_more::Display;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::{
    controllers::{
        middlewares::UserMiddleware,
        schema::{AppState, ResponseLogin, ResponseMe, ResponseSignUp},
    },
    models::verify_tokens::VerifyTokensDBError,
    services::users::UsersServiceError,
};

#[derive(Deserialize, Validate, ToSchema)]
pub struct SignUpData {
    username: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 8, max = 512))]
    password: String,
    #[validate(must_match(other = "password"))]
    repeat_password: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginData {
    username: String,
    #[validate(length(min = 8, max = 512))]
    password: String,
}

#[derive(Deserialize)]
pub struct VerifyData {
    pub verify_key: Uuid,
    pub redirect: String,
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
            Self::UsersServiceError(service_err) => match service_err {
                UsersServiceError::GenericDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                UsersServiceError::UsersDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                UsersServiceError::InvalidUserPassword => StatusCode::UNAUTHORIZED,
                UsersServiceError::VerifyTokensDBError(err) => match err {
                    VerifyTokensDBError::VerifyTokenNotFound => StatusCode::NOT_FOUND,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                UsersServiceError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        return match self {
            Self::InvalidInputData(_errors) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"error": "Invalid input data", "status": "invalid_data"}).to_string()),
            Self::UsersServiceError(service_err) => match service_err {
                UsersServiceError::GenericDBError(_) => HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
                UsersServiceError::UsersDBError(_) => HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
                UsersServiceError::VerifyTokensDBError(err) => match err {
                    VerifyTokensDBError::VerifyTokenNotFound => {
                        HttpResponse::build(self.status_code())
                            .insert_header(ContentType::json())
                            .body(
                                json!({"error": "Verify token not found", "status": "not_found"})
                                    .to_string(),
                            )
                    }
                    _ => HttpResponse::build(self.status_code())
                        .insert_header(ContentType::json())
                        .body(
                            json!({"error": "Unknown error", "status": "unknown_error"})
                                .to_string(),
                        ),
                },
                UsersServiceError::InvalidUserPassword => HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(
                        json!({"error": "Invalid credentials", "status": "invalid_credentials"})
                            .to_string(),
                    ),
                UsersServiceError::UnknownError => HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
            },
            Self::UnknownError => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
        };
    }
}

#[utoipa::path(
    params(("X-API-AUTH-TOKEN" = Uuid, Header, description = "Auth token"),),
    responses(
    (status = OK, description = "OK", body = ResponseMe),
    (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
    (status = INTERNAL_SERVER_ERROR, description = "Internal server error", body = ErrorResponse)
    ),
    tag = "users"
)]
#[get("/api/v1/users/me")]
pub async fn me(
    user: UserMiddleware,
    data: web::Data<AppState>,
) -> Result<web::Json<ResponseMe>, UsersError> {
    let user_id = user.user_id;
    let r = web::block(move || data.users_service.get_user_by_id(user_id))
        .await
        .unwrap();

    match r {
        Ok(user) => Ok(web::Json(ResponseMe {
            status: "success".to_string(),
            data: user,
        })),
        Err(err) => Err(UsersError::UsersServiceError(err)),
    }
}

#[utoipa::path(
    responses(
    (status = OK, description = "OK", body = ResponseSignUp),
    (status = BAD_REQUEST, description = "Data is not valid", body = ErrorResponse)
    ),
tag = "users"
)]
#[post("/api/v1/users/signup")]
pub async fn signup(
    data: web::Json<SignUpData>,
    state: web::Data<AppState>,
) -> Result<web::Json<ResponseSignUp>, UsersError> {
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
                Ok(user) => Ok(web::Json(ResponseSignUp {
                    status: "success".to_string(),
                    data: user,
                })),
                Err(err) => Err(UsersError::UsersServiceError(err)),
            }
        }
        Err(err) => Err(UsersError::InvalidInputData(err)),
    }
}

#[utoipa::path(
params(("verify_key" = Uuid, Query, description = "Verify token"),("redirect" = String, Query, description = "Redirect link")),
responses(
(status = OK, description = "OK", body = ResponseLogin),
(status = NOT_FOUND, description = "Verify token not found or expired", body = ErrorResponse)
),
tag = "users")
]
#[get("/api/v1/users/verify")]
pub async fn verify_user(
    query_data: web::Query<VerifyData>,
    state: web::Data<AppState>,
) -> Result<web::Redirect, UsersError> {
    let verify_key = query_data.verify_key;
    let r = web::block(move || state.users_service.verify_user(verify_key))
        .await
        .unwrap();

    match r {
        Ok(()) => Ok(web::Redirect::to(query_data.redirect.clone()).permanent()),
        Err(err) => Err(UsersError::UsersServiceError(err)),
    }
}

#[utoipa::path(
responses(
(status = OK, description = "OK", body = ResponseLogin),
(status = BAD_REQUEST, description = "Data is not valid", body = ErrorResponse)
),
tag = "users")
]
#[post("/api/v1/users/login")]
pub async fn login(
    data: web::Json<LoginData>,
    state: web::Data<AppState>,
) -> Result<web::Json<ResponseLogin>, UsersError> {
    match data.validate() {
        Ok(_) => {
            let inner_state = state.clone();
            let r = web::block(move || {
                inner_state
                    .users_service
                    .authenticate_user(data.username.clone(), data.password.clone())
            })
            .await
            .unwrap();

            match r {
                Ok(user) => {
                    let now = Utc::now();
                    let iat = now.timestamp() as usize;
                    let exp =
                        (now + Duration::minutes(state.jwt_maxage as i64)).timestamp() as usize;
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
                    Ok(web::Json(ResponseLogin {
                        status: "success".to_string(),
                        data: token,
                    }))
                }
                Err(err) => Err(UsersError::UsersServiceError(err)),
            }
        }
        Err(err) => Err(UsersError::InvalidInputData(err)),
    }
}
