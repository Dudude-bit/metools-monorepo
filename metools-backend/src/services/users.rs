use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher as _, PasswordVerifier as _};
use derive_more::Display;
use rand_core::OsRng;
use uuid::Uuid;

use crate::models::users::{
    get_user_by_id, get_user_by_username, insert_new_user, is_user_verified,
    GetUserByUsernameReturn, UserReturn, UsersDBError,
};
use crate::models::DBPool;

#[derive(Debug, Display)]
pub enum UsersServiceError {
    UsersDBError(UsersDBError),
    InvalidUserPassword,
    UnknownError,
}

#[derive(Clone)]
pub struct UsersService {
    pool: DBPool,
}

impl UsersService {
    pub fn init(pool: DBPool) -> Self {
        Self { pool }
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
            Ok(hashed_password) => {
                let r = insert_new_user(
                    &mut self.pool.get().unwrap(),
                    username,
                    email,
                    hashed_password.to_string(),
                );
                match r {
                    Ok(user) => Ok(user),
                    Err(err) => Err(UsersServiceError::UsersDBError(err)),
                }
            }
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

    pub fn get_user_is_verified(&self, user_id: Uuid) -> Result<bool, UsersServiceError> {
        let r = is_user_verified(&mut self.pool.get().unwrap(), user_id);

        match r {
            Ok(is_user_verified) => Ok(is_user_verified),
            Err(err) => Err(UsersServiceError::UsersDBError(err)),
        }
    }
}
