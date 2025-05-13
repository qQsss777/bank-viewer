use oul_bank_macro::New;
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
#[derive(Debug, Deserialize, Serialize)]
pub struct CheckUser {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, New)]
pub struct BDDUser {
    id: i32,
    firstname: String,
    name: String,
    username: String,
    create_at: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let bdd_user: BDDUser =
            BDDUser::new(1, "firstname".to_owned(), "name".to_owned(), "username".to_owned(), 0);
        assert_eq!(bdd_user.name, "name");

        let bdd_user =
            CheckUser { username: "username".to_owned(), password: "username".to_owned() };
        assert_eq!(bdd_user.username, "username");
    }
}
