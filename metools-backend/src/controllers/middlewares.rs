use std::{future::Future, pin::Pin};

use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorInternalServerError, ErrorUnauthorized},
    web, Error as ActixWebError, FromRequest, HttpRequest,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use surrealdb::sql::Id;

use crate::controllers::{schema::AppState, users::users::TokenClaims};

pub struct UserMiddleware {
    pub user_id: String,
}

impl FromRequest for UserMiddleware {
    type Error = ActixWebError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let data = req.app_data::<web::Data<AppState>>().unwrap();
            let token = req.headers().get("X-API-AUTH-TOKEN");
            if token.is_none() {
                return Err(ErrorUnauthorized(web::Json(
                    json!({"status": "unauthorized", "error": "Unauthorized"}),
                )));
            }
            let token = token.unwrap().to_str();

            if token.is_err() {
                return Err(ErrorUnauthorized(web::Json(
                    json!({"status": "unauthorized", "error": "Unauthorized"}),
                )));
            }

            let claims = match decode::<TokenClaims>(
                token.unwrap(),
                &DecodingKey::from_secret(data.jwt_secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(c) => c.claims,
                Err(_) => {
                    return Err(ErrorUnauthorized(web::Json(
                        json!({"status": "unauthorized", "error": "Unauthorized"}),
                    )));
                }
            };

            let user_id = claims.sub.to_string();
            let is_user_verified = data
                .users_service
                .get_user_is_verified(user_id.clone())
                .await;
            match is_user_verified {
                Ok(is_user_verified) => {
                    if !is_user_verified {
                        return Err(ErrorForbidden(web::Json(
                            json!({"status": "not_verified", "error": "User is not verified"}),
                        )));
                    }
                }
                Err(_) => {
                    return Err(ErrorInternalServerError(web::Json(
                        json!({"status": "unknown_error", "error": "Unknown error"}),
                    )));
                }
            }
            Ok(Self {
                user_id: user_id.to_owned(),
            })
        })
    }
}
