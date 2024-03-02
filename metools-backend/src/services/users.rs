use crate::models::users::{insert_new_user, User};
use crate::models::DBPool;
use crate::services::users::UsersServiceError::UnknownError;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher as _};
use diesel::PgConnection;
use rand_core::OsRng;

pub enum UsersServiceError {
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
        return match hashed_password {
            Ok(hashed_password) => {
                let user = insert_new_user(
                    &mut self.pool.get().unwrap(),
                    username,
                    email,
                    hashed_password.to_string(),
                );
                Err(UnknownError)
            }
            Err(err) => {
                log::error!("Error on hashing password while registering user: {err}");
                Err(UnknownError)
            }
        };
    }
}
