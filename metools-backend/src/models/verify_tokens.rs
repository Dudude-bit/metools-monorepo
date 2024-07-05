use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Id, Connection, Error, Response, Surreal};
use uuid::Uuid;

const TABLE_NAME: &str = "rzd_tasks";

#[derive(Debug, Display)]
pub enum VerifyTokensDBError {
    VerifyTokenNotFound,
    UnknownError(Error),
}

#[derive(Serialize)]
pub struct NewVerifyToken {
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
    pub user_id: Id,
}

#[derive(Deserialize, Clone)]
pub struct VerifyTokenReturn {
    pub id: Id,
    pub user_id: Id,
    pub created_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
}

pub async fn create_verify_token<T: Connection>(
    conn: &Surreal<T>,
    token_value: Uuid,
    valid_until_value: DateTime<Utc>,
    user_id: Id,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let verify_token = NewVerifyToken {
        token: token_value,
        valid_until: valid_until_value,
        user_id: user_id,
    };
    let r: Result<Vec<VerifyTokenReturn>, Error> =
        conn.insert(TABLE_NAME).content(verify_token).await;

    match r {
        Ok(verify_tokens) => Ok(verify_tokens[0].clone()),
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}

pub async fn get_verify_token_by_value<T: Connection>(
    conn: &Surreal<T>,
    token_value: Id,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let r: QueryResult<VerifyTokenReturn> = verify_tokens
        .filter(
            token
                .eq(token_value)
                .and(valid_until.gt(chrono::offset::Utc::now())),
        )
        .select(VerifyTokenReturn::as_select())
        .first(conn);

    match r {
        Ok(verify_token) => Ok(verify_token),
        Err(err) => match err {
            Error::NotFound => Err(VerifyTokensDBError::VerifyTokenNotFound),
            _ => Err(VerifyTokensDBError::UnknownError(err)),
        },
    }
}

pub async fn delete_verify_token_by_id<T: Connection>(
    conn: &Surreal<T>,
    verify_token_id: Id,
) -> Result<(), VerifyTokensDBError> {
    use crate::schema::verify_tokens::dsl::*;

    let r: QueryResult<usize> =
        diesel::delete(verify_tokens.filter(id.eq(verify_token_id))).execute(conn);
    match r {
        Ok(usize) => {
            if usize == 0 {
                return Err(VerifyTokensDBError::VerifyTokenNotFound);
            }
            Ok(())
        }
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}

pub async fn delete_expired_verify_tokens<T: Connection>(
    conn: Surreal<T>,
) -> Result<usize, VerifyTokensDBError> {
    let r: Result<Response, Error> = conn
        .query("DELETE type::table($table) WHERE valid_until <= $valid_until")
        .bind((("table", TABLE_NAME), ("valid_until", chrono::Utc::now())))
        .await;
    match r {
        Ok(c) => Ok(c),
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}
