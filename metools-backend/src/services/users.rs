use crate::models::users::{insert_new_user, User, UsersDBError};
use crate::models::DBPool;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher as _};
use derive_more::Display;

use rand_core::OsRng;

#[derive(Debug, Display)]
pub enum UsersServiceError {
    UsersDBError(UsersDBError),
    UnknownError,
}

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
    ) -> Result<User, UsersServiceError> {
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
                    Ok(user) => return Ok(user),
                    Err(err) => Err(UsersServiceError::UsersDBError(err)),
                }
            }
            Err(err) => {
                log::error!("Error on hashing password while registering user: {err}");
                Err(UsersServiceError::UnknownError)
            }
        }
    }
}
