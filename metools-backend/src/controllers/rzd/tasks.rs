use std::collections::HashMap;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use derive_more::Display;
use serde::Deserialize;
use serde_json::json;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::controllers::middlewares::UserMiddleware;
use crate::controllers::schema::AppState;
use crate::controllers::schema::{
    ResponseCreateTask, ResponseDeleteTaskByIdForUser, ResponseListTasks,
};
use crate::models::rzd::tasks::TasksDBError;
use crate::services::tasks::TasksServiceError;

#[derive(Debug, Display)]
enum TasksError {
    TasksServiceError(TasksServiceError),
}

impl ResponseError for TasksError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::TasksServiceError(error) => match error {
                TasksServiceError::TasksDBError(error) => match error {
                    TasksDBError::NoDeletedTask => StatusCode::NOT_FOUND,
                    TasksDBError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
                },
                TasksServiceError::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        match self {
            Self::TasksServiceError(error) => match error {
                TasksServiceError::TasksDBError(error) => match error {
                    TasksDBError::NoDeletedTask => HttpResponse::build(self.status_code())
                        .insert_header(ContentType::json())
                        .body(
                            json!({"error": "Task not found", "status": "not_found"}).to_string(),
                        ),
                    TasksDBError::UnknownError => HttpResponse::build(self.status_code())
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

#[derive(Deserialize, ToSchema)]
pub struct CreateTaskData {
    task_type: String,
    data: HashMap<String, String>,
}

#[derive(Deserialize)]
struct DeleteTaskData {
    task_id: Uuid,
}

#[utoipa::path(
    responses(
    (status = OK, description = "OK", body = ResponseListTasks)
    ),
tag = "tasks"
)]
#[get("/api/v1/rzd/tasks")]
pub async fn list_tasks(
    _: UserMiddleware,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<web::Json<ResponseListTasks>, TasksError> {
    let user_id = *req.extensions().get::<Uuid>().unwrap();

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
    responses(
    (status = OK, description = "OK", body = ResponseCreateTask)
    ),
tag = "tasks"
)]
#[post("/api/v1/rzd/tasks")]
pub async fn create_task(
    _: UserMiddleware,
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Json<CreateTaskData>,
) -> Result<web::Json<ResponseCreateTask>, TasksError> {
    let user_id = *req.extensions().get::<Uuid>().unwrap();

    let r = web::block(move || {
        state
            .tasks_service
            .create_task_for_user(user_id, data.task_type.clone(), data.data.clone())
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

#[utoipa::path(
    responses(
    (status = OK, description = "OK", body = ResponseDeleteTaskByIdForUser)
    ),
    params(("task_id" = Uuid, Path, description = "Task id"),),
    tag = "tasks"
)]
#[delete("/api/v1/rzd/tasks/{task_id}")]
pub async fn delete_task_by_id_for_user(
    _: UserMiddleware,
    req: HttpRequest,
    state: web::Data<AppState>,
    data: web::Path<DeleteTaskData>,
) -> Result<web::Json<ResponseDeleteTaskByIdForUser>, TasksError> {
    let user_id = *req.extensions().get::<Uuid>().unwrap();

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
    responses(
    (status = OK, description = "OK", body = ResponseDeleteAllTasksForUser)
    ),
    tag = "tasks"
)]
#[delete("/api/v1/rzd/tasks")]
pub async fn delete_all_tasks_for_user(
    _: UserMiddleware,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<web::Json<ResponseDeleteTaskByIdForUser>, TasksError> {
    let user_id = *req.extensions().get::<Uuid>().unwrap();

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