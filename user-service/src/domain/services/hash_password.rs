use crate::domain::value_objects::error::PasswordHashError;

pub trait HashPasswordService: Send + Sync {
    fn check_password(&self, from_db: &str, from_client: &str) -> Result<(), String>;
    fn hash_password(&self, password: &str) -> Result<String, PasswordHashError>;
}
