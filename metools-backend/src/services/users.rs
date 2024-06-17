use argon2::{
    password_hash::SaltString, Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier as _,
};
use chrono::Days;
use derive_more::Display;
use diesel::Connection;
use rand_core::OsRng;
use uuid::Uuid;

use crate::{
    models::{
        users::{
            get_user_by_id, get_user_by_username, insert_new_user, is_user_verified,
            set_user_verified, GetUserByUsernameReturn, UserReturn, UsersDBError,
        },
        verify_tokens::{
            create_verify_token, delete_verify_token_by_id, get_verify_token_by_value,
            VerifyTokensDBError,
        },
        DBPool,
    },
    services::mailer::MailerService,
};

#[derive(Debug, Display)]
pub enum UsersServiceError {
    GenericDBError(diesel::result::Error),
    UsersDBError(UsersDBError),
    VerifyTokensDBError(VerifyTokensDBError),
    InvalidUserPassword,
    UnknownError,
}

impl From<diesel::result::Error> for UsersServiceError {
    fn from(e: diesel::result::Error) -> Self {
        UsersServiceError::GenericDBError(e)
    }
}

#[derive(Clone)]
pub struct UsersService {
    pool: DBPool,
    mailer: MailerService,
}

impl UsersService {
    pub fn init(pool: DBPool, mailer: MailerService) -> Self {
        Self { pool, mailer }
    }

    pub fn register_user(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<UserReturn, UsersServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default().hash_password(password.as_bytes(), &salt);
        match hashed_password {
            Ok(hashed_password) => self.pool.get().unwrap().transaction(|connection| {
                let r_user = insert_new_user(
                    connection,
                    username,
                    email.clone(),
                    hashed_password.to_string(),
                );
                if r_user.is_err() {
                    return Err(UsersServiceError::UsersDBError(r_user.err().unwrap()));
                }
                let r_user = r_user.unwrap();

                let r_verify_token = create_verify_token(
                    connection,
                    Uuid::new_v4(),
                    chrono::offset::Utc::now()
                        .checked_add_days(Days::new(1)) // verify token valid for 1 day
                        .unwrap(),
                    r_user.id,
                );

                if r_verify_token.is_err() {
                    return Err(UsersServiceError::VerifyTokensDBError(
                        r_verify_token.err().unwrap(),
                    ));
                }

                let r_verify_token = r_verify_token.unwrap();

                let r_email = self
                    .mailer
                    .send_verification_mail(email, r_verify_token.token);
                match r_email {
                    Ok(_) => Ok(r_user),
                    Err(err) => {
                        log::error!("Error on sending email: {err}");
                        Err(UsersServiceError::UnknownError)
                    }
                }
            }),
            Err(err) => {
                log::error!("Error on hashing password while registering user: {err}");
                Err(UsersServiceError::UnknownError)
            }
        }
    }

    pub fn authenticate_user(
        &self,
        username: String,
        password: String,
    ) -> Result<GetUserByUsernameReturn, UsersServiceError> {
        let r = get_user_by_username(&mut self.pool.get().unwrap(), username);

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

    pub fn get_user_by_id(&self, user_id: Uuid) -> Result<UserReturn, UsersServiceError> {
        let r = get_user_by_id(&mut self.pool.get().unwrap(), user_id);

        match r {
            Ok(user) => Ok(user),
            Err(err) => Err(UsersServiceError::UsersDBError(err)),
        }
    }
    pub fn verify_user(&self, token: Uuid) -> Result<(), UsersServiceError> {
        self.pool.get().unwrap().transaction(|connection| {
            let verify_token = get_verify_token_by_value(connection, token);
            if verify_token.is_err() {
                return Err(UsersServiceError::VerifyTokensDBError(
                    verify_token.err().unwrap(),
                ));
            }
            let verify_token = verify_token.unwrap();

            let r_set_user_verified = set_user_verified(connection, verify_token.user_id);
            if r_set_user_verified.is_err() {
                return Err(UsersServiceError::UsersDBError(
                    r_set_user_verified.err().unwrap(),
                ));
            }

            let r = delete_verify_token_by_id(connection, verify_token.id);

            match r {
                Ok(()) => Ok(()),
                Err(err) => Err(UsersServiceError::VerifyTokensDBError(err)),
            }
        })
    }

    pub fn get_user_is_verified(&self, user_id: Uuid) -> Result<bool, UsersServiceError> {
        let r = is_user_verified(&mut self.pool.get().unwrap(), user_id);

        match r {
            Ok(is_user_verified) => Ok(is_user_verified),
            Err(err) => Err(UsersServiceError::UsersDBError(err)),
        }
    }
}
