use std::collections::HashMap;

use chrono;
use derive_more::Display;
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{schema::rzd_tasks::dsl::rzd_tasks};

#[derive(Debug, Display)]
pub enum TasksDBError {
    NoDeletedTask,
    UnknownError,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::rzd_tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct NewTask {
    id: Uuid,
    user_id: Uuid,
    type_: String,
    data: Value,
}
#[derive(Queryable, Selectable, Serialize, Deserialize, Debug, ToSchema)]
#[diesel(table_name = crate::schema::rzd_tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    pub id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub type_: String,
    pub data: Value,
    pub user_id: Uuid,
}

pub fn insert_new_task(
    conn: &mut PgConnection,
    task_user_id: Uuid,
    task_type: String,
    task_data: HashMap<String, String>,
) -> Result<Task, TasksDBError> {
    use crate::schema::rzd_tasks::dsl::*;

    let new_task = NewTask {
        id: Uuid::new_v4(),
        user_id: task_user_id,
        type_: task_type,
        data: json!(task_data),
    };

    let r: QueryResult<Task> = diesel::insert_into(rzd_tasks)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn);

    match r {
        Ok(tasks) => Ok(tasks),
        Err(_) => Err(TasksDBError::UnknownError),
    }
}

pub fn list_all_tasks(conn: &mut PgConnection) -> Result<Vec<Task>, Error> {
    rzd_tasks.select(Task::as_select()).load(conn)
}

pub fn list_all_users_tasks(
    conn: &mut PgConnection,
    task_user_id: Uuid,
) -> Result<Vec<Task>, TasksDBError> {
    use crate::schema::rzd_tasks::dsl::*;

    let r: QueryResult<Vec<Task>> = rzd_tasks
        .filter(user_id.eq(task_user_id))
        .select(Task::as_select())
        .load(conn);

    match r {
        Ok(tasks) => Ok(tasks),
        Err(err) => {
            log::error!("Error on list all user tasks: {err}");
            Err(TasksDBError::UnknownError)
        }
    }
}

pub fn delete_task_by_id_for_user(
    conn: &mut PgConnection,
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
        Err(err) => Err(TasksDBError::UnknownError),
    }
}
