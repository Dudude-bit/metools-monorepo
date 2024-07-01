use derive_more::Display;
use serde::Serialize;
use surrealdb::{sql::Thing, Connection, Surreal};
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum UsersDBError {
    UnknownError(Error),
}

pub struct UserReturn {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}
pub struct GetUserByUsernameReturn {
    pub id: Thing,
    pub username: String,
    pub password: String,
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn insert_new_user<T: Connection>(
    conn: &Surreal<T>,
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
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub fn get_user_by_username<T: Connection>(
    conn: &Surreal<T>,
    user_username: String,
) -> Result<GetUserByUsernameReturn, UsersDBError> {
    let r: QueryResult<GetUserByUsernameReturn> = users
        .filter(username.eq(user_username))
        .select(GetUserByUsernameReturn::as_select())
        .get_result(conn);

    match r {
        Ok(user) => Ok(user),
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub fn get_user_by_id<T: Connection>(
    conn: Surreal<T>,
    user_id: Uuid,
) -> Result<UserReturn, UsersDBError> {
    let r: QueryResult<UserReturn> = users
        .filter(id.eq(user_id))
        .select(UserReturn::as_select())
        .get_result(conn);

    match r {
        Ok(user) => Ok(user),
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub fn is_user_verified<T: Connection>(
    conn: Surreal<T>,
    user_id: Uuid,
) -> Result<bool, UsersDBError> {
    let r: QueryResult<bool> = users
        .filter(id.eq(user_id))
        .select(is_verified)
        .get_result(conn);

    match r {
        Ok(is_user_verified) => Ok(is_user_verified),
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub fn set_user_verified<T: Connection>(
    conn: Surreal<T>,
    user_id: Uuid,
) -> Result<(), UsersDBError> {
    let r: QueryResult<usize> = diesel::update(users)
        .filter(id.eq(user_id))
        .set(is_verified.eq(true))
        .execute(conn);

    match r {
        Ok(_) => Ok(()),
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}
