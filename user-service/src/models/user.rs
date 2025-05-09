use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    firstname: String,
    name: String,
    username: String,
    password: String,
    email: String,
}

impl CreateUser {
    pub fn get_keys(&self) -> String {
        String::from("firstname, name, username, password, email")
    }

    pub fn get_values(&self) -> String {
        format!(
            "'{}', '{}', '{}', '{}', '{}'",
            self.firstname, self.name, self.username, self.password, self.email
        )
    }
}

#[derive(Debug, Serialize)]
pub struct BDDUser {
    id: i32,
    firstname: String,
    name: String,
    username: String,
    create_at: i32,
}
