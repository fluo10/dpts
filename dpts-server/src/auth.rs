use argon2::{
    Argon2, 
    PasswordHasher,
    PasswordVerifier,
    password_hash::{
        Salt,
        SaltString,
        PasswordHash,
        rand_core::OsRng,
    },
};
use chrono::format::parse;
use crate::error::Error;
use tokio::sync::OnceCell;

pub fn try_hash_password(password: &str) -> Result<String, Error> {
    let mut rng = OsRng::default();
    let salt_string= SaltString::generate(&mut rng);
    let salt = salt_string.as_salt();
    let hash = Argon2::default().hash_password(password.as_bytes(), salt).or(Err(Error::PasswordHash("Hashing password with salt".to_string())))?;
    Ok(hash.to_string())
}

pub fn try_verify_password(password: &str, password_hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(password_hash).or(Err(Error::PasswordHash("Failed to parse password hash string".to_string())))?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[cfg(test)]
mod tests {
    use argon2::password_hash::rand_core::OsRng;

    use super::*;
    #[test]
    fn test_password() {
        let valid_password = "valid";
        let invalid_password = "invalid";
        let hash = try_hash_password(valid_password).unwrap();
        assert!(try_verify_password(valid_password, &hash).unwrap());
        assert!(!try_verify_password(invalid_password, &hash).unwrap());
    }
}