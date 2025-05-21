use crate::domains::services::jwt::JWTService;
use oul_bank_macro::New;

#[derive(New)]
pub struct JWTServiceImpl {
    key: String,
}

impl JWTService for JWTServiceImpl {
    fn check_password(&self, from_db: &String, from_client: &String) -> bool {
        true
    }

    fn encrypted(&self, password: &String) -> String {
        String::new()
    }

    fn generate_token(&self) -> String {
        String::new()
    }

    fn decode_token(&self) -> String {
        String::new()
    }

    fn unvalidate_token(&self) -> String {
        String::new()
    }
}
