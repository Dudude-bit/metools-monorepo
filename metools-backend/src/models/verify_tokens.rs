use chrono::{DateTime, Utc};
use derive_more::Display;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum VerifyTokensDBError {
    UnknownError(Error),
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::verify_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewVerifyToken {
    pub id: Uuid,
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
    pub user_id: Uuid,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::verify_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VerifyTokenReturn {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub token: Uuid,
    pub user_id: Uuid,
}

pub fn create_verify_token(
    conn: &mut PgConnection,
    token_value: Uuid,
    valid_until_value: DateTime<Utc>,
    user_id_value: Uuid,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    use crate::schema::verify_tokens::dsl::*;
    let verify_token = NewVerifyToken {
        id: Uuid::new_v4(),
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

pub fn get_verify_token_by_value(
    conn: &mut PgConnection,
    token_value: Uuid,
) -> Result<VerifyTokenReturn, VerifyTokensDBError> {
    use crate::schema::verify_tokens::dsl::*;

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
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}
