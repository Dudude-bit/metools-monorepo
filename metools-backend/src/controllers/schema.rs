use serde::Serialize;
use utoipa::ToSchema;

use crate::models::users::UserReturn;
use crate::services::users::UsersService;

#[derive(Clone)]
pub struct AppState {
    pub users_service: UsersService,
    pub jwt_secret: String,
    pub jwt_maxage: i32,
}

#[derive(Serialize, ToSchema)]
#[aliases(ResponseMe = Response<UserReturn>, ResponseLogin = Response<String>)]
pub struct Response<T: Serialize> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    status: String,
    error: String,
}
