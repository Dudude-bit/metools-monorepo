use serde::Serialize;
use utoipa::ToSchema;

use crate::{
    controllers::{
        rzd::tasks::ResponseListTasksData,
        users::users::{ResponseMeData, ResponseSignupData},
    },
    models::rzd::tasks::Task,
    services::{tasks::TasksService, users::UsersService},
};

#[derive(Clone)]
pub struct AppState {
    pub users_service: UsersService,
    pub tasks_service: TasksService,
    pub jwt_secret: String,
    pub jwt_maxage: usize,
}

#[derive(Serialize, ToSchema)]
#[aliases(ResponseMe = Response<ResponseMeData>,
    ResponseLogin = Response<String>,
    ResponseSignup = Response<ResponseSignupData>,
    ResponseListTasks = Response<Vec<ResponseListTasksData>>,
    ResponseCreateTask = Response<Task>,
    ResponseDeleteTaskByIdForUser = Response<String>,
    ResponseDeleteAllTasksForUser = Response<String>)]
pub struct Response<T: Serialize> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    status: String,
    error: String,
}
