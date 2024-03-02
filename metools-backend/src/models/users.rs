use crate::schema::users::dsl::users;
use derive_more::Display;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum UsersDBError {
    UnknownError,
}

#[derive(Queryable, Selectable, Serialize)]
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
) -> Result<User, UsersDBError> {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        username: user_username,
        email: user_email,
        password: user_password,
    };

    let r: QueryResult<User> = diesel::insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn);

    return match r {
        Ok(user) => Ok(user),
        Err(err) => {
            log::error!("Error on inserting user {err}");
            Err(UsersDBError::UnknownError)
        }
    };
}
