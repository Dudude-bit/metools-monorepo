use std::collections::HashMap;

use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::{
    sql::{Datetime, Thing},
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
    user: Thing,
    #[serde(rename = "type")]
    type_: String,
    data: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub struct Task {
    pub id: Thing,
    pub created_at: Datetime,
    #[serde(rename = "type")]
    pub type_: String,
    pub data: HashMap<String, String>,
    pub user: Thing,
}

const TABLE_NAME: &str = "rzd_tasks";

pub async fn insert_new_task<T: Connection>(
    conn: Surreal<T>,
    user_id: Thing,
    type_: String,
    task_data: HashMap<String, String>,
) -> Result<Task, TasksDBError> {
    let new_task = NewTask {
        user: user_id,
        type_,
        data: task_data,
    };

    let r: Result<Vec<Task>, Error> = conn.create(TABLE_NAME).content(new_task).await;

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
    let r: Result<Response, Error> = conn.query("SELECT id, created_at, type, data, user FROM type::table($table) WHERE user = <record>$user_id").bind(
        json!(
            {
                "table": TABLE_NAME,
                "user_id":  user_id.to_string()
            }
        )
    ).await;

    match r {
        Ok(mut tasks) => Ok(tasks.take::<Vec<Task>>(0).unwrap().clone()),
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}

pub async fn delete_task_by_id_for_user<T: Connection>(
    conn: Surreal<T>,
    user_id: Thing,
    task_id: Thing,
) -> Result<(), TasksDBError> {
    let r = conn.query("count(DELETE type::table($table) WHERE user = <record>$user_id AND id = <record>$task_id RETURN BEFORE)").bind(
        json!(
            {
                "table": TABLE_NAME,
                "user_id": user_id.to_string(),
                "task_id": task_id.to_string()
            }
        )
    ).await;
    match r {
        Ok(mut r) => {
            let num_deleted_tasks = r.take::<Vec<usize>>(0).unwrap()[0];
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
    user_id: Thing,
) -> Result<usize, TasksDBError> {
    let r = conn
        .query("count(DELETE type::table($table) WHERE user = <record>$user_id RETURN BEFORE)")
        .bind(json!(
            {
                "table": TABLE_NAME,
                "user_id": user_id.to_string()
            }
        ))
        .await;
    match r {
        Ok(mut r) => {
            let surreal_response = r.take::<Vec<usize>>(0).unwrap()[0];
            Ok(surreal_response)
        }
        Err(err) => Err(TasksDBError::UnknownError(err)),
    }
}
