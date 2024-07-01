use std::collections::HashMap;

use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::{sql::Thing, Connection, Error, Surreal};
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum TasksDBError {
    NoDeletedTask,
    UnknownError(Error),
}

#[derive(Serialize)]
struct NewTask {
    user_id: Thing,
    type_: String,
    data: Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Thing,
    pub created_at: DateTime<Utc>,
    pub type_: String,
    pub data: Value,
    pub user_id: Thing,
}

const TABLE_NAME: &str = "users";

pub async fn insert_new_task<T: Connection>(
    conn: Surreal<T>,
    task_user_id: Thing,
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
    user_id: Thing,
) -> Result<Vec<Task>, TasksDBError> {
    let r = conn.query("SELECT id, created_at, type_, data, user_id FROM type::table($table) WHERE user_id = $user_id").bind((("table", TABLE_NAME), ("user_id", user_id))).await;

    match r {
        Ok(tasks) => Ok(tasks.take::(0).unwrap().clone()),
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn delete_task_by_id_for_user<T: Connection>(
    conn: Surreal<T>,
    task_user_id: Uuid,
    task_id: Uuid,
) -> Result<(), TasksDBError> {
    use crate::schema::rzd_tasks::dsl::*;
    let r = diesel::delete(rzd_tasks.filter(user_id.eq(task_user_id).and(id.eq(task_id))))
        .execute(conn);
    match r {
        Ok(r) => {
            if r == 0 {
                Err(TasksDBError::NoDeletedTask)
            } else {
                Ok(())
            }
        }
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn delete_all_tasks_for_user<T: Connection>(
    conn: Surreal<T>,
    task_user_id: Uuid,
) -> Result<usize, TasksDBError> {
    use crate::schema::rzd_tasks::dsl::*;
    let r = diesel::delete(rzd_tasks.filter(user_id.eq(task_user_id))).execute(conn);
    match r {
        Ok(r) => Ok(r),
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}
