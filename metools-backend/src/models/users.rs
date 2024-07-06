use derive_more::Display;
use serde::{Deserialize, Serialize};
use surrealdb::{opt::PatchOp, sql::Id, Connection, Error, Response, Surreal};

const TABLE_NAME: &str = "rzd_tasks";

#[derive(Debug, Display)]
pub enum UsersDBError {
    UserNotFound,
    UnknownError(Error),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserReturn {
    pub id: String,
    pub username: String,
    pub is_verified: bool,
    pub email: String,
    pub role: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn insert_new_user<T: Connection>(
    conn: &Surreal<T>,
    user_username: String,
    user_email: String,
    user_password: String,
) -> Result<UserReturn, UsersDBError> {
    let new_user = NewUser {
        username: user_username,
        email: user_email,
        password: user_password,
    };

    let r: Result<Vec<UserReturn>, Error> = conn.insert(TABLE_NAME).content(new_user).await;

    match r {
        Ok(users) => Ok(users[0].clone()),
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub async fn get_user_by_username<T: Connection>(
    conn: &Surreal<T>,
    username: String,
) -> Result<UserReturn, UsersDBError> {
    let r: Result<Response, Error> = conn
        .query("SELECT username, is_verified, email, role, password FROM type::table($table) WHERE username = $username")
        .bind((("table", TABLE_NAME), ("username", username)))
        .await;

    match r {
        Ok(mut response) => {
            let user_take = response.take(0);
            match user_take {
                Ok(user_option) => match user_option {
                    Some(user) => Ok(user),
                    None => Err(UsersDBError::UserNotFound),
                },
                Err(err) => Err(UsersDBError::UnknownError(err)),
            }
        }
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub async fn get_user_by_id<T: Connection>(
    conn: Surreal<T>,
    user_id: String,
) -> Result<UserReturn, UsersDBError> {
    let r: Result<Option<UserReturn>, Error> = conn.select((TABLE_NAME, user_id)).await;

    match r {
        Ok(user_option) => match user_option {
            Some(user) => Ok(user),
            None => Err(UsersDBError::UserNotFound),
        },
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}

pub async fn is_user_verified<T: Connection>(
    conn: Surreal<T>,
    user_id: String,
) -> Result<bool, UsersDBError> {
    match get_user_by_id(conn, user_id).await {
        Ok(user) => Ok(user.is_verified),
        Err(err) => Err(err),
    }
}

pub async fn set_user_verified<T: Connection>(
    conn: &Surreal<T>,
    user_id: String,
) -> Result<(), UsersDBError> {
    let r: Result<Option<UserReturn>, Error> = conn
        .update((TABLE_NAME, user_id))
        .patch(PatchOp::replace("/is_verified", true))
        .await;

    match r {
        Ok(_) => Ok(()),
        Err(err) => Err(UsersDBError::UnknownError(err)),
    }
}
