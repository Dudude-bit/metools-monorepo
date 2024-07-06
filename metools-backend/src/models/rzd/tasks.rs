use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::{
    sql::{Id, Thing},
    Connection, Error, Response, Surreal,
};
use utoipa::ToSchema;

#[derive(Debug, Display)]
pub enum TasksDBError {
    NoDeletedTask,
    UnknownError(Error),
}

#[derive(Serialize)]
struct NewTask {
    user_id: String,
    type_: String,
    data: Value,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct Task {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub type_: String,
    pub data: Value,
    pub user_id: String,
}

const TABLE_NAME: &str = "rzd_tasks";

pub async fn insert_new_task<T: Connection>(
    conn: Surreal<T>,
    task_user_id: String,
    task_type: String,
    task_data: HashMap<String, String>,
) -> Result<Task, TasksDBError> {
    let new_task = NewTask {
        user_id: task_user_id,
        type_: task_type,
        data: json!(task_data),
    };

    let r: Result<Vec<Task>, Error> = conn.insert(TABLE_NAME).content(new_task).await;

    match r {
        Ok(tasks) => Ok(tasks[0].clone()),
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn list_all_tasks<T: Connection>(conn: Surreal<T>) -> Result<Vec<Task>, TasksDBError> {
    let r: Result<Vec<Task>, Error> = conn.select(TABLE_NAME).await;

    match r {
        Ok(tasks) => Ok(tasks),
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn list_all_users_tasks<T: Connection>(
    conn: Surreal<T>,
    user_id: String,
) -> Result<Vec<Task>, TasksDBError> {
    let r: Result<Response, Error> = conn.query("SELECT id, created_at, type_, data, user_id FROM type::table($table) WHERE user_id = $user_id").bind((("table", TABLE_NAME), ("user_id", user_id))).await;

    match r {
        Ok(mut tasks) => Ok(tasks.take::<Vec<Task>>(0).unwrap().clone()),
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn delete_task_by_id_for_user<T: Connection>(
    conn: Surreal<T>,
    user_id: String,
    task_id: String,
) -> Result<(), TasksDBError> {
    let r = conn.query("SELECT count() as deleted_tasks FROM (DELETE type::table($table) WHERE user_id = $user_id AND id = $task_id RETURN BEFORE)").bind((("table", TABLE_NAME), ("user_id", user_id), ("task_id", task_id))).await;
    match r {
        Ok(mut r) => {
            let surreal_response = r.take::<Vec<HashMap<String, usize>>>(0).unwrap()[0].clone();
            let num_deleted_tasks = *surreal_response.get("deleted_tasks").unwrap();
            if num_deleted_tasks == 0 {
                return Err(TasksDBError::NoDeletedTask);
            }
            Ok(())
        }
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn delete_all_tasks_for_user<T: Connection>(
    conn: Surreal<T>,
    user_id: String,
) -> Result<usize, TasksDBError> {
    let r = conn
        .query("SELECT count() as deleted_tasks FROM (DELETE type::table($table) WHERE user_id = $user_id RETURN BEFORE)")
        .bind((("table", TABLE_NAME), ("user_id", user_id)))
        .await;
    match r {
        Ok(mut r) => {
            let surreal_response = r.take::<Vec<HashMap<String, usize>>>(0).unwrap()[0].clone();
            Ok(*surreal_response.get("deleted_tasks").unwrap())
        }
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}
