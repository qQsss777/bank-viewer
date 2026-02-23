use crate::domain::services::hash_password::HashPasswordService;
use crate::domain::value_objects::error::PasswordHashError;
use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier, SaltString},
};

pub struct HashPasswordServiceImpl {
    pub salt: SaltString,
}

impl HashPasswordService for HashPasswordServiceImpl {
    fn check_password(&self, from_db: &str, from_client: &str) -> Result<(), String> {
        let argon2: Argon2<'_> = Argon2::default();
        let parsed_hash = PasswordHash::new(from_db)
            .map_err(|_| "Invalid password hash stored in DB".to_string())?;
        argon2
            .verify_password(&from_client.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid password".to_string())
    }

    fn hash_password(&self, password: &str) -> Result<String, PasswordHashError> {
        let argon2: Argon2<'_> = Argon2::default();
        let password_hashed = argon2
            .hash_password(password.as_bytes(), &self.salt)
            .map_err(|_| PasswordHashError::HashFailed)?
            .to_string();
        Ok(password_hashed)
    }
}
