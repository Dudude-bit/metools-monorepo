use std::future::{ready, Ready};

use crate::controllers::schema::AppState;
use crate::controllers::users::users::TokenClaims;
use actix_web::error::ErrorUnauthorized;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{web, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;

pub struct UserMiddleware;

impl FromRequest for UserMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();

        let token = req.headers().get("X-API-AUTH-TOKEN");

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(web::Json(
                json!({"status": "unauthorized", "error": "Unauthorized"}),
            ))));
        }

        let token = token.unwrap().to_str();

        if token.is_err() {
            return ready(Err(ErrorUnauthorized(web::Json(
                json!({"status": "unauthorized", "error": "Unauthorized"}),
            ))));
        }

        let claims = match decode::<TokenClaims>(
            token.unwrap(),
            &DecodingKey::from_secret(data.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(web::Json(
                    json!({"status": "unauthorized", "error": "Unauthorized"}),
                ))));
            }
        };

        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());
        ready(Ok(Self))
    }
}
