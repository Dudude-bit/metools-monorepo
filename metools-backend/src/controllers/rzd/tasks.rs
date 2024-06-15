use std::collections::HashMap;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpResponse, ResponseError};
use derive_more::Display;
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidateArgs, ValidationError, ValidationErrors};

use crate::controllers::middlewares::UserMiddleware;
use crate::controllers::schema::AppState;
use crate::controllers::schema::{
    ResponseCreateTask, ResponseDeleteTaskByIdForUser, ResponseListTasks,
};
use crate::models::rzd::tasks::TasksDBError;
use crate::services::tasks::TasksServiceError;

#[derive(Debug, Display)]
enum TasksError {
    InvalidInputData(ValidationErrors),
    TasksServiceError(TasksServiceError),
}

impl ResponseError for TasksError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidInputData(_) => StatusCode::BAD_REQUEST,
            Self::TasksServiceError(error) => match error {
                TasksServiceError::TasksDBError(error) => match error {
                    TasksDBError::NoDeletedTask => StatusCode::NOT_FOUND,
                    TasksDBError::UnknownError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                },
                TasksServiceError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::InvalidInputData(_errors) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"error": "Invalid input data", "status": "invalid_data"}).to_string()),
            Self::TasksServiceError(error) => match error {
                TasksServiceError::TasksDBError(error) => match error {
                    TasksDBError::NoDeletedTask => HttpResponse::build(self.status_code())
                        .insert_header(ContentType::json())
                        .body(
                            json!({"error": "Task not found", "status": "not_found"}).to_string(),
                        ),
                    TasksDBError::UnknownError(_) => HttpResponse::build(self.status_code())
                        .insert_header(ContentType::json())
                        .body(
                            json!({"error": "Unknown error", "status": "unknown_error"})
                                .to_string(),
                        ),
                },
                TasksServiceError::UnknownError => HttpResponse::build(self.status_code())
                    .insert_header(ContentType::json())
                    .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
            },
        }
    }
}

pub struct ValidateCreateTaskDataContext(String);

fn validate_create_task_data(
    data: &HashMap<String, String>,
    context: &ValidateCreateTaskDataContext,
) -> Result<(), ValidationError> {
    match context.0.as_str() {
        "day" => {
            if data.keys().len() != 3 {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("from_point_code") {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("to_point_code") {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("date") {
                return Err(ValidationError::new("bad_task_data"));
            }
            Ok(())
        }
        "train" => {
            if data.keys().len() != 5 {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("from_point_code") {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("to_point_code") {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("date") {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("time") {
                return Err(ValidationError::new("bad_task_data"));
            }
            if !data.contains_key("tnum") {
                return Err(ValidationError::new("bad_task_data"));
            }
            Ok(())
        }
        &_ => Err(ValidationError::new("unknown_task_type")),
    }
}

#[derive(Deserialize, ToSchema, Validate)]
#[validate(context = ValidateCreateTaskDataContext)]
pub struct CreateTaskData {
    task_type: String,
    #[validate(custom(function = "validate_create_task_data", use_context))]
    data: HashMap<String, String>,
}

#[derive(Deserialize)]
struct DeleteTaskData {
    task_id: Uuid,
}

#[utoipa::path(
    params(("X-API-AUTH-TOKEN" = Uuid, Header, description = "Auth token"),),
    responses(
    (status = OK, description = "OK", body = ResponseListTasks),
    (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
    (status = INTERNAL_SERVER_ERROR, description = "INTERNAL_SERVER_ERROR", body = ErrorResponse)
    ),
tag = "tasks"
)]
#[get("/api/v1/rzd/tasks")]
pub async fn list_tasks(
    user: UserMiddleware,
    state: web::Data<AppState>,
) -> Result<web::Json<ResponseListTasks>, TasksError> {
    let user_id = user.user_id;

    let r = web::block(move || state.tasks_service.list_tasks_for_user(user_id))
        .await
        .unwrap();

    match r {
        Ok(tasks) => Ok(web::Json(ResponseListTasks {
            status: "success".to_string(),
            data: tasks,
        })),
        Err(err) => Err(TasksError::TasksServiceError(err)),
    }
}

#[utoipa::path(
    params(("X-API-AUTH-TOKEN" = Uuid, Header, description = "Auth token"),),
    responses(
    (status = OK, description = "OK", body = ResponseCreateTask),
    (status = BAD_REQUEST, description = "Data is not valid", body = ErrorResponse),
    (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
    (status = INTERNAL_SERVER_ERROR, description = "INTERNAL_SERVER_ERROR", body = ErrorResponse)
    ),
tag = "tasks"
)]
#[post("/api/v1/rzd/tasks")]
pub async fn create_task(
    user: UserMiddleware,
    state: web::Data<AppState>,
    data: web::Json<CreateTaskData>,
) -> Result<web::Json<ResponseCreateTask>, TasksError> {
    let user_id = user.user_id;
    match data.validate_with_args(&ValidateCreateTaskDataContext(data.task_type.clone())) {
        Ok(_) => {
            let r = web::block(move || {
                state.tasks_service.create_task_for_user(
                    user_id,
                    data.task_type.clone(),
                    data.data.clone(),
                )
            })
            .await
            .unwrap();

            match r {
                Ok(task) => Ok(web::Json(ResponseCreateTask {
                    status: "success".to_string(),
                    data: task,
                })),
                Err(err) => Err(TasksError::TasksServiceError(err)),
            }
        }
        Err(err) => Err(TasksError::InvalidInputData(err)),
    }
}

#[utoipa::path(
    params(("task_id" = Uuid, Path, description = "Task id"),("X-API-AUTH-TOKEN" = Uuid, Header, description = "Auth token"),),
    responses(
    (status = OK, description = "OK", body = ResponseDeleteTaskByIdForUser),
    (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
    (status = NOT_FOUND, description = "Task not found for user", body = ErrorResponse),
    (status = INTERNAL_SERVER_ERROR, description = "INTERNAL_SERVER_ERROR", body = ErrorResponse)
    ),
    tag = "tasks"
)]
#[delete("/api/v1/rzd/tasks/{task_id}")]
pub async fn delete_task_by_id_for_user(
    user: UserMiddleware,
    state: web::Data<AppState>,
    data: web::Path<DeleteTaskData>,
) -> Result<web::Json<ResponseDeleteTaskByIdForUser>, TasksError> {
    let user_id = user.user_id;

    let r = web::block(move || {
        state
            .tasks_service
            .delete_task_by_id_for_user(user_id, data.task_id)
    })
    .await
    .unwrap();

    match r {
        Ok(_) => Ok(web::Json(ResponseDeleteTaskByIdForUser {
            status: "success".to_string(),
            data: String::from("success"),
        })),
        Err(err) => Err(TasksError::TasksServiceError(err)),
    }
}

#[utoipa::path(
    params(("X-API-AUTH-TOKEN" = Uuid, Header, description = "Auth token"),),
    responses(
    (status = OK, description = "OK", body = ResponseDeleteAllTasksForUser),
    (status = UNAUTHORIZED, description = "Unauthorized", body = ErrorResponse),
    (status = INTERNAL_SERVER_ERROR, description = "INTERNAL_SERVER_ERROR", body = ErrorResponse)
    ),
    tag = "tasks"
)]
#[delete("/api/v1/rzd/tasks")]
pub async fn delete_all_tasks_for_user(
    user: UserMiddleware,
    state: web::Data<AppState>,
) -> Result<web::Json<ResponseDeleteTaskByIdForUser>, TasksError> {
    let user_id = user.user_id;

    let r = web::block(move || state.tasks_service.delete_all_tasks_for_user(user_id))
        .await
        .unwrap();

    match r {
        Ok(r) => Ok(web::Json(ResponseDeleteTaskByIdForUser {
            status: "success".to_string(),
            data: format!("Successfull deleted {r} tasks"),
        })),
        Err(err) => Err(TasksError::TasksServiceError(err)),
    }
}
