use derive_more::Display;
use diesel::{prelude::*, result::Error};
use uuid::Uuid;

#[derive(Debug, Display)]
pub enum VerifyTokensDBError {
    UnknownError(Error),
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::verify_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VerifyTokenReturn {
    pub id: Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub valid_until: chrono::NaiveDateTime,
    pub token: Uuid,
    pub user_id: Uuid,
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
                .and(valid_until.lt(chrono::offset::Utc::now())),
        )
        .select(VerifyTokenReturn::as_select())
        .first(conn);

    match r {
        Ok(verify_token) => Ok(verify_token),
        Err(err) => Err(VerifyTokensDBError::UnknownError(err)),
    }
}
