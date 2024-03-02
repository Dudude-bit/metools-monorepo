use crate::schema::users::dsl::users;
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    id: Uuid,
    username: String,
    email: String,
    password: String,
}

pub fn insert_new_user(
    conn: &mut PgConnection,
    user_username: String,
    user_email: String,
    user_password: String,
) {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        username: user_username,
        email: user_email,
        password: user_password,
    };

    let _r: QueryResult<User> = diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn);
}
