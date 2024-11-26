use argon2::{
    password_hash::{rand_core, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<(), argon2::password_hash::Error> {
    let argon2 = Argon2::default();
    let password_hash = PasswordHash::new(&password_hash)?;
    let is_valid = argon2.verify_password(password.as_bytes(), &password_hash)?;
    Ok(is_valid)
}
