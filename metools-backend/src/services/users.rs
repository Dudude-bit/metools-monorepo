use std::ptr::hash;
use argon2::{Argon2, PasswordHasher as _};
use argon2::password_hash::SaltString;
use diesel::PgConnection;
use rand_core::OsRng;

use crate::models::users::*;

pub fn register_user(conn: &mut PgConnection, username: String, email: String, password: String) -> Result<> {
    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(password.as_bytes(), &salt);
    return match hashed_password {
        Ok(hashed_password) => {
            register_user(conn, username, email, hashed_password.to_string())
        }
        Err(err) => {

        }
    }
}