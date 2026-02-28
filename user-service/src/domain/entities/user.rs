use oul_bank_macro::New;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, New)]
pub struct User {
    pub id: Option<String>,
    pub firstname: String,
    pub name: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl User {
    pub fn get_keys(&self) -> String {
        String::from("firstname, name, username, password, email")
    }
    pub fn get_values(&self) -> String {
        format!(
            "'{}', '{}', '{}', '{}', '{}'",
            self.firstname, self.name, self.username, self.password, self.email
        )
    }
    pub fn change_password(&mut self, hashed: String) {
        self.password = hashed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_user() {
        let bdd_user: User = User::new(
            None,
            "firstname".to_owned(),
            "name".to_owned(),
            "username".to_owned(),
            "password".to_owned(),
            "a@example.com".to_owned(),
        );
        assert_eq!(bdd_user.name, "name");
    }
}
