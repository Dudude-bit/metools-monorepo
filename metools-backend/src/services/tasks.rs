use std::collections::HashMap;

use derive_more::Display;
use surrealdb::sql::Thing;

use crate::{
    config::DBConfig,
    models::rzd::tasks::{
        delete_all_tasks_for_user, delete_task_by_id_for_user, insert_new_task,
        list_all_users_tasks, Task, TasksDBError,
    },
};

#[derive(Debug, Display)]
pub enum TasksServiceError {
    TasksDBError(TasksDBError),
    UnknownError,
}
#[derive(Clone)]
pub struct TasksService {
    db: DBConfig,
}

impl TasksService {
    pub fn init(db: DBConfig) -> Self {
        Self { db }
    }
    pub async fn list_tasks_for_user(
        &self,
        user_id: Thing,
    ) -> Result<Vec<Task>, TasksServiceError> {
        let tasks = list_all_users_tasks(self.db.get_connection().await, user_id).await;
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(err) => Err(TasksServiceError::TasksDBError(err)),
        }
    }

    pub async fn create_task_for_user(
        &self,
        user_id: Thing,
        task_type: String,
        task_data: HashMap<String, String>,
    ) -> Result<Task, TasksServiceError> {
        let r = insert_new_task(
            self.db.get_connection().await,
            user_id,
            task_type,
            task_data,
        )
        .await;

        match r {
            Ok(task) => Ok(task),
            Err(err) => Err(TasksServiceError::TasksDBError(err)),
        }
    }

    pub async fn delete_task_by_id_for_user(
        &self,
        user_id: Thing,
        task_id: Thing,
    ) -> Result<(), TasksServiceError> {
        let r = delete_task_by_id_for_user(self.db.get_connection().await, user_id, task_id).await;

        match r {
            Ok(()) => Ok(()),
            Err(err) => Err(TasksServiceError::TasksDBError(err)),
        }
    }

    pub async fn delete_all_tasks_for_user(
        &self,
        user_id: Thing,
    ) -> Result<usize, TasksServiceError> {
        let r = delete_all_tasks_for_user(self.db.get_connection().await, user_id).await;

        match r {
            Ok(r) => Ok(r),
            Err(err) => Err(TasksServiceError::TasksDBError(err)),
        }
    }
}
