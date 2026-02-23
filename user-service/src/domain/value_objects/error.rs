use std::fmt;

#[derive(Debug)]
pub enum PasswordHashError {
    HashFailed,
}

impl fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasswordHashError::HashFailed => write!(f, "Password hashing failed"),
        }
    }
}
