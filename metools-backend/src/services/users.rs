use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier as _,
};
use chrono::Days;
use derive_more::Display;
use rand_core::OsRng;
use surrealdb::{sql::Thing, Error};
use uuid::Uuid;

use crate::{
    config::DBConfig,
    models::{
        users::{
            get_user_by_id, get_user_by_username, insert_new_user, is_user_verified,
            set_user_verified, UserReturn, UsersDBError,
        },
        verify_tokens::{
            create_verify_token, delete_verify_token_by_id, get_verify_token_by_value,
            VerifyTokensDBError,
        },
    },
    services::mailer::MailerService,
};

#[derive(Debug, Display)]
pub enum UsersServiceError {
    UsersDBError(UsersDBError),
    VerifyTokensDBError(VerifyTokensDBError),
    InvalidUserPassword,
    CommitError(Error),
    UnknownError,
}

#[derive(Clone)]
pub struct UsersService {
    db: DBConfig,
    mailer: MailerService,
}

impl UsersService {
    pub fn init(db: DBConfig, mailer: MailerService) -> Self {
        Self { db, mailer }
    }

    pub async fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<UserReturn, UsersServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default().hash_password(password.as_bytes(), &salt);
        match hashed_password {
            Ok(hashed_password) => {
                let conn = self.db.get_connection().await;
                let r_user =
                    insert_new_user(&conn, username, email.clone(), hashed_password.to_string())
                        .await;
                if r_user.is_err() {
                    return Err(UsersServiceError::UsersDBError(r_user.err().unwrap()));
                }
                let r_user = r_user.unwrap();

                let r_verify_token = create_verify_token(
                    &conn,
                    Uuid::new_v4(),
                    chrono::offset::Utc::now()
                        .checked_add_days(Days::new(1)) // verify token valid for 1 day
                        .unwrap(),
                    r_user.id.clone(),
                )
                .await;

                if r_verify_token.is_err() {
                    return Err(UsersServiceError::VerifyTokensDBError(
                        r_verify_token.err().unwrap(),
                    ));
                }

                let r_verify_token = r_verify_token.unwrap();

                let r_email = self
                    .mailer
                    .send_verification_mail(email, r_verify_token.token.0);
                match r_email {
                    Ok(_) => Ok(r_user),
                    Err(err) => {
                        log::error!("Error on sending email: {err}");
                        Err(UsersServiceError::UnknownError)
                    }
                }
            }
            Err(err) => {
                log::error!("Error on hashing password while registering user: {err}");
                Err(UsersServiceError::UnknownError)
            }
        }
    }

    pub async fn authenticate_user(
        &self,
        username: String,
        password: String,
    ) -> Result<UserReturn, UsersServiceError> {
        let r = get_user_by_username(&self.db.get_connection().await, username).await;

        match r {
            Ok(user) => {
                let parsed_hash = PasswordHash::new(user.password.as_str()).unwrap();
                let is_valid = Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .map_or(false, |_| true);
                if !is_valid {
                    return Err(UsersServiceError::InvalidUserPassword);
                }
                Ok(user)
            }
            Err(err) => Err(UsersServiceError::UsersDBError(err)),
        }
    }

    pub async fn get_user_by_id(&self, user_id: Thing) -> Result<UserReturn, UsersServiceError> {
        let r = get_user_by_id(self.db.get_connection().await, user_id).await;

        match r {
            Ok(user) => Ok(user),
            Err(err) => Err(UsersServiceError::UsersDBError(err)),
        }
    }
    pub async fn verify_user(&self, token: Uuid) -> Result<(), UsersServiceError> {
        let conn = self.db.get_connection().await;
        let verify_token = get_verify_token_by_value(&conn, token).await;
        if verify_token.is_err() {
            return Err(UsersServiceError::VerifyTokensDBError(
                verify_token.err().unwrap(),
            ));
        }
        let verify_token = verify_token.unwrap();

        let r_set_user_verified = set_user_verified(&conn, verify_token.user).await;
        if r_set_user_verified.is_err() {
            return Err(UsersServiceError::UsersDBError(
                r_set_user_verified.err().unwrap(),
            ));
        }

        let r = delete_verify_token_by_id(&conn, verify_token.id).await;

        match r {
            Ok(()) => Ok(()),
            Err(err) => Err(UsersServiceError::VerifyTokensDBError(err)),
        }
    }

    pub async fn get_user_is_verified(&self, user_id: Thing) -> Result<bool, UsersServiceError> {
        let r = is_user_verified(self.db.get_connection().await, user_id).await;

        match r {
            Ok(is_user_verified) => Ok(is_user_verified),
            Err(err) => Err(UsersServiceError::UsersDBError(err)),
        }
    }
}
