use chrono::{DateTime, Utc};
use derive_more::Display;
use surrealdb::{sql::Thing, Connection, Surreal};
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum VerifyTokensDBError {
    VerifyTokenNotFound,
    UnknownError(Error),
}

pub struct NewVerifyToken {
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
    pub user_id: Uuid,
}

pub struct VerifyTokenReturn {
    pub id: Thing,
    pub created_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
    pub user_id: Uuid,
}

pub fn create_verify_token<T: Connection>(
    conn: &Surreal<T>,
    token_value: Uuid,
    valid_until_value: DateTime<Utc>,
    user_id_value: Uuid,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let verify_token = NewVerifyToken {
        token: token_value,
        valid_until: valid_until_value,
        user_id: user_id_value,
    };
    let r: QueryResult<VerifyTokenReturn> = diesel::insert_into(verify_tokens)
        .values(verify_token)
        .returning(VerifyTokenReturn::as_returning())
        .get_result(conn);

    match r {
        Ok(verify_token) => Ok(verify_token),
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}

pub fn get_verify_token_by_value<T: Connection>(
    conn: Surreal<T>,
    token_value: Uuid,
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

pub fn delete_verify_token_by_id<T: Connection>(
    conn: Surreal<T>,
    verify_token_id: Uuid,
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

pub fn delete_expired_verify_tokens(
    conn: &Surreal<Connection>,
) -> Result<usize, VerifyTokensDBError> {
    use crate::schema::verify_tokens::dsl::*;
    let r: QueryResult<usize> =
        diesel::delete(verify_tokens.filter(valid_until.le(chrono::Utc::now()))).execute(conn);
    match r {
        Ok(c) => Ok(c),
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}
