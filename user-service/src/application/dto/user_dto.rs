use crate::domain::entities::user::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct CreateUserDTO {
    firstname: String,
    name: String,
    username: String,
    password: String,
    email: String,
}

impl TryFrom<&CreateUserDTO> for User {
    type Error = String;

    fn try_from(dto: &CreateUserDTO) -> Result<Self, Self::Error> {
        if dto.email.is_empty() {
            return Err("Email cannot be empty".into());
        }
        if dto.firstname.is_empty() {
            return Err("Firstname cannot be empty".into());
        }
        if dto.name.is_empty() {
            return Err("Name cannot be empty".into());
        }
        if dto.username.is_empty() {
            return Err("Username cannot be empty".into());
        }
        if dto.password.is_empty() {
            return Err("Password cannot be empty".into());
        }
        let user = User::new(
            None,
            dto.firstname.clone(),
            dto.name.clone(),
            dto.username.clone(),
            dto.password.clone(),
            dto.email.clone(),
        );
        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user_from_dto_success() {
        let dto = CreateUserDTO {
            firstname: "John".to_string(),
            name: "Doe".to_string(),
            username: "johndoe".to_string(),
            password: "securepassword".to_string(),
            email: "john@example.com".to_string(),
        };

        let user_result = User::try_from(&dto);

        assert!(user_result.is_ok());

        let user = user_result.unwrap();
        assert_eq!(user.firstname, "John");
        assert_eq!(user.name, "Doe");
        assert_eq!(user.username, "johndoe");
        assert_eq!(user.email, "john@example.com");
        assert_eq!(user.password, "securepassword"); // si tu ne hashe pas encore ici
    }

    #[test]
    fn test_create_user_from_dto_missing_email() {
        let dto = CreateUserDTO {
            firstname: "John".to_string(),
            name: "Doe".to_string(),
            username: "johndoe".to_string(),
            password: "securepassword".to_string(),
            email: "".to_string(),
        };

        let user_result = User::try_from(&dto);

        assert!(user_result.is_err());
        assert_eq!(user_result.unwrap_err(), "Email cannot be empty");
    }

    #[test]
    fn test_create_user_from_dto_missing_password() {
        let dto = CreateUserDTO {
            firstname: "John".to_string(),
            name: "Doe".to_string(),
            username: "johndoe".to_string(),
            password: "".to_string(),
            email: "john@example.com".to_string(),
        };

        let user_result = User::try_from(&dto);

        assert!(user_result.is_err());
        assert_eq!(user_result.unwrap_err(), "Password cannot be empty");
    }
}
