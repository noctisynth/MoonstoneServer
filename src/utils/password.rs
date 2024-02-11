use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::exceptions::MoonstoneException;

pub fn hash_password(password: &str) -> Result<String, MoonstoneException> {
    let salt = SaltString::generate(&mut OsRng);

    match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(password) => Ok(password.to_string()),
        Err(error) => Err(MoonstoneException::PasswordHashFailed { error }),
    }
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(&password_hash) {
        Ok(password_hash) => password_hash,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
