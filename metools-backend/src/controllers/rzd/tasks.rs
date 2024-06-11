use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, web, HttpMessage, HttpRequest, HttpResponse, ResponseError};
use derive_more::Display;
use serde_json::json;
use uuid::Uuid;

use crate::controllers::middlewares::UserMiddleware;
use crate::controllers::schema::AppState;
use crate::controllers::schema::ResponseListTasks;
use crate::services::tasks::TasksServiceError;

#[derive(Debug, Display)]
enum TasksError {
    TasksServiceError(TasksServiceError),
}

impl ResponseError for TasksError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::TasksServiceError(_error) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse<BoxBody> {
        return match self {
            Self::TasksServiceError(_error) => HttpResponse::build(self.status_code())
                .insert_header(ContentType::json())
                .body(json!({"error": "Unknown error", "status": "unknown_error"}).to_string()),
        };
    }
}

#[utoipa::path(
    responses(
    (status = OK, description = "OK", body = ResponseListTasks)
    ),
tag = "tasks"
)]
#[get("/")]
pub async fn list_tasks(
    _: UserMiddleware,
    req: HttpRequest,
    data: web::Data<AppState>,
) -> Result<web::Json<ResponseListTasks>, TasksError> {
    let user_id = *req.extensions().get::<Uuid>().unwrap();

    let r = web::block(move || data.tasks_service.list_tasks_for_user(user_id))
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
