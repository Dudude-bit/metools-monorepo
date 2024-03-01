use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    id: Uuid,
}


pub fn insert_new_user(conn: &mut PgConnection, username: String, email: String, password: String) {

}