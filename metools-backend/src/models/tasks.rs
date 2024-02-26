use std::collections::HashMap;
use diesel::{Insertable, Queryable, Selectable, PgConnection, RunQueryDsl as _, SelectableHelper as _, QueryResult};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::schema::rzd_tasks::dsl::rzd_tasks;

#[derive(Deserialize, Serialize, Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::rzd_tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    id: Uuid,
    user_id: Uuid,
    type_: String,
    data: Value,
}

pub fn insert_new_task(
    conn: &mut PgConnection,
    task_user_id: Uuid,
    task_type: String,
    task_data: HashMap<String, String>
){
    use crate::schema::rzd_tasks::dsl::*;

    let new_task = Task {
        id: Uuid::new_v4(),
        user_id: task_user_id,
        type_: task_type,
        data: json!(task_data),
    };


    let r: QueryResult<Task> = diesel::insert_into(rzd_tasks)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn);

    println!("{:?}", r)
}