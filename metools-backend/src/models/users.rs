use derive_more::Display;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::schema::users::dsl::users;

#[derive(Debug, Display)]
pub enum UsersDBError {
    UnknownError,
}

#[derive(Queryable, Selectable, Serialize, ToSchema)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserReturn {
    id: Uuid,
    username: String,
    email: String,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GetUserByUsernameReturn {
    pub id: Uuid,
    pub username: String,
    pub password: String,
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
) -> Result<UserReturn, UsersDBError> {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        username: user_username,
        email: user_email,
        password: user_password,
    };

    let r: QueryResult<UserReturn> = diesel::insert_into(users)
        .values(&new_user)
        .returning(UserReturn::as_returning())
        .get_result(conn);

    match r {
        Ok(user) => Ok(user),
        Err(err) => {
            log::error!("Error on inserting user {err}");
            Err(UsersDBError::UnknownError)
        }
    }
}

pub fn get_user_by_username(
    conn: &mut PgConnection,
    user_username: String,
) -> Result<GetUserByUsernameReturn, UsersDBError> {
    use crate::schema::users::dsl::*;

    let r: QueryResult<GetUserByUsernameReturn> = users
        .filter(username.eq(user_username))
        .select(GetUserByUsernameReturn::as_select())
        .get_result(conn);

    match r {
        Ok(user) => Ok(user),
        Err(err) => {
            log::error!("Error on get user by username {err}");
            Err(UsersDBError::UnknownError)
        }
    }
}

pub fn get_user_by_id(conn: &mut PgConnection, user_id: Uuid) -> Result<UserReturn, UsersDBError> {
    use crate::schema::users::dsl::*;

    let r: QueryResult<UserReturn> = users
        .filter(id.eq(user_id))
        .select(UserReturn::as_select())
        .get_result(conn);

    match r {
        Ok(user) => Ok(user),
        Err(err) => {
            log::error!("Error on get user by id {err}");
            Err(UsersDBError::UnknownError)
        }
    }
}

pub fn is_user_verified(conn: &mut PgConnection, user_id: Uuid) -> Result<bool, UsersDBError> {
    use crate::schema::users::dsl::*;

    let r: QueryResult<bool> = users
        .filter(id.eq(user_id))
        .select(is_verified)
        .get_result(conn);

    match r {
        Ok(is_user_verified) => Ok(is_user_verified),
        Err(err) => {
            log::error!("Error on check if user is_verified by id {err}");
            Err(UsersDBError::UnknownError)
        }
    }
}
