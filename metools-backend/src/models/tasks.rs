use std::collections::HashMap;
use diesel::{Insertable, Queryable, Selectable, PgConnection, RunQueryDsl as _, SelectableHelper as _, QueryResult};

use crate::schema::rzd_tasks::dsl::rzd_tasks;

#[derive(Debug, Insertable, Queryable, Selectable)]
#[diesel(table_name = crate::schema::rzd_tasks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Task {
    id: String,
    type_: String,
    data: HashMap<String, String>,
}

pub fn insert_new_task(
    conn: &mut PgConnection,
    type_: String,
    data: HashMap<String, String>
) -> Result<Task, String>{
    use crate::schema::rzd_tasks::dsl::*;

    let uid = format!("{}", uuid::Uuid::new_v4());
    let new_task = Task {
        id: uid,
        type_,
        data,
    };

    let r: QueryResult<Task> = diesel::insert_into(rzd_tasks)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn);


}