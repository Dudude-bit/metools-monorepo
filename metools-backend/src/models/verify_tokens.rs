use std::collections::HashMap;

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
    pub user_id: String,
}

#[derive(Deserialize, Clone)]
pub struct VerifyTokenReturn {
    pub id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
}

pub async fn create_verify_token<T: Connection>(
    conn: &Surreal<T>,
    token: Uuid,
    valid_until: DateTime<Utc>,
    user_id: String,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let verify_token = NewVerifyToken {
        token,
        valid_until,
        user_id,
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
    token: String,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    let r: Result<Response, Error> = conn.query("SELECT id, user_id, created_at, valid_until, token FROM type::table($table) WHERE token = $token AND valid_until > $valid_until").bind((("table", TABLE_NAME), ("token", token), ("valid_until", chrono::Utc::now()))).await;
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
    verify_token_id: String,
) -> Result<(), VerifyTokensDBError> {
    let r = conn.query("SELECT count() as deleted_verify_tokens FROM (DELETE type::table($table) WHERE id = $verify_token_id RETURN BEFORE)").bind((("table", TABLE_NAME), ("verify_token_id", verify_token_id))).await;
    match r {
        Ok(mut r) => {
            let surreal_response = r.take::<Vec<HashMap<String, usize>>>(0).unwrap()[0].clone();
            let num_deleted_tasks = *surreal_response.get("deleted_verify_tokens").unwrap();
            if num_deleted_tasks == 0 {
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
        .query("SELECT count() as deleted_verify_tokens FROM (DELETE type::table($table) WHERE valid_until <= $valid_until)")
        .bind((("table", TABLE_NAME), ("valid_until", chrono::Utc::now())))
        .await;
    match r {
        Ok(mut r) => {
            let surreal_response = r.take::<Vec<HashMap<String, usize>>>(0).unwrap()[0].clone();
            Ok(*surreal_response.get("deleted_verify_tokens").unwrap())
        }
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}
