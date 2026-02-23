use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckUser {
    pub username: String,
    pub password: String,
}

mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let check_user =
            CheckUser { username: "username".to_owned(), password: "username".to_owned() };
        assert_eq!(check_user.username, "username");
    }
}
