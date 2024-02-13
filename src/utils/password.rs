use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::exceptions::Exception;

pub(crate) fn hash_password(password: &str) -> Result<String, Exception> {
    let salt = SaltString::generate(&mut OsRng);

    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(password) => Ok(password.to_string()),
        Err(error) => Err(Exception::PasswordHashFailed { error }),
    }
}

pub(crate) fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(password_hash) => password_hash,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
