use serde::Serialize;
use utoipa::ToSchema;

use crate::models::rzd::tasks::Task;
use crate::models::users::UserReturn;
use crate::services::tasks::TasksService;
use crate::services::users::UsersService;

#[derive(Clone)]
pub struct AppState {
    pub users_service: UsersService,
    pub tasks_service: TasksService,
    pub jwt_secret: String,
    pub jwt_maxage: i32,
}

#[derive(Serialize, ToSchema)]
#[aliases(ResponseMe = Response<UserReturn>, ResponseLogin = Response<String>, ResponseSignUp = Response<UserReturn>, ResponseListTasks = Response<Vec<Task>>, ResponseCreateTask = Response<Task>, ResponseDeleteTaskByIdForUser = Response<String>, ResponseDeleteAllTasksForUser = Response<String>)]
pub struct Response<T: Serialize> {
    pub status: String,
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    status: String,
    error: String,
}
