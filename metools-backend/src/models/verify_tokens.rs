use chrono::{DateTime, Utc};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::{
    sql::{Datetime, Thing, Uuid as DBUuid},
    Connection, Error, Response, Surreal,
};
use uuid::Uuid;

use super::generic::Record;

const TABLE_NAME: &str = "verify_tokens";

#[derive(Debug, Display)]
pub enum VerifyTokensDBError {
    VerifyTokenNotFound,
    UnknownError(Error),
}

#[derive(Serialize)]
pub struct NewVerifyToken {
    pub valid_until: Datetime,
    pub token: DBUuid,
    pub user: Thing,
}

#[derive(Deserialize, Clone, Debug)]
pub struct VerifyTokenReturn {
    pub id: Thing,
    pub created_at: Datetime,
    pub user: Thing,
    pub valid_until: Datetime,
    pub token: DBUuid,
}

pub async fn create_verify_token<T: Connection>(
    conn: &Surreal<T>,
    token: Uuid,
    valid_until: DateTime<Utc>,
    user_id: Thing,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let verify_token = NewVerifyToken {
        token: DBUuid::from(token),
        valid_until: Datetime::from(valid_until),
        user: user_id,
    };
    let r: Result<Vec<VerifyTokenReturn>, Error> =
        conn.create(TABLE_NAME).content(verify_token).await;

    match r {
        Ok(verify_tokens) => Ok(verify_tokens[0].clone()),
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}

pub async fn get_verify_token_by_value<T: Connection>(
    conn: &Surreal<T>,
    token: Uuid,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let r: Result<Response, Error> = conn.query("SELECT id, user_id, created_at, valid_until, token, user FROM type::table($table) WHERE token = <uuid>$token_value AND valid_until > <datetime>$valid_until").bind(json!({
        "table": TABLE_NAME,
        "token_value": DBUuid::from(token),
        "valid_until": Datetime::from(chrono::Utc::now())
    })).await;
    match r {
        Ok(mut verify_token) => {
            let verify_token_take = verify_token.take::<Vec<VerifyTokenReturn>>(0);
            if verify_token_take.is_err() {
                return Err(VerifyTokensDBError::UnknownError(
                    verify_token_take.err().unwrap(),
                ));
            }
            let verify_token_take = verify_token_take.unwrap();
            if verify_token_take.is_empty() {
                Err(VerifyTokensDBError::VerifyTokenNotFound)
            } else {
                Ok(verify_token_take[0].clone())
            }
        }
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}

pub async fn delete_verify_token_by_id<T: Connection>(
    conn: &Surreal<T>,
    verify_token_id: Thing,
) -> Result<(), VerifyTokensDBError> {
    let r = conn.delete::<Option<Record>>(verify_token_id).await;
    match r {
        Ok(r) => {
            if r.is_none() {
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
        .query("count(DELETE type::table($table) WHERE valid_until <= <datetime>$valid_until RETURN BEFORE)")
        .bind(json!(
            {
                "table": TABLE_NAME,
                "valid_until": Datetime::from(chrono::Utc::now())
            }
        ))
        .await;
    match r {
        Ok(mut r) => {
            let surreal_response = r.take::<Vec<usize>>(0).unwrap()[0];
            Ok(surreal_response)
        }
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}
