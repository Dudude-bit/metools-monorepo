use derive_more::Display;
use uuid::Uuid;

use crate::models::rzd::tasks::{list_all_users_tasks, Task, TasksDBError};
use crate::models::DBPool;

#[derive(Debug, Display)]
pub enum TasksServiceError {
    TasksDBError(TasksDBError),
    UnknownError,
}
#[derive(Clone)]
pub struct TasksService {
    pool: DBPool,
}

impl TasksService {
    pub fn init(pool: DBPool) -> Self {
        Self { pool }
    }
    pub fn list_tasks_for_user(&self, user_id: Uuid) -> Result<Vec<Task>, TasksServiceError> {
        let tasks = list_all_users_tasks(&mut self.pool.get().unwrap(), user_id);
        match tasks {
            Ok(tasks) => Ok(tasks),
            Err(err) => Err(TasksServiceError::TasksDBError(err)),
        }
    }
}
