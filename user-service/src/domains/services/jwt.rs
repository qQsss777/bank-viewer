use crate::domains::models::token::Token;

pub trait JWTService: Send + Sync {
    fn check_password(&self, from_db: &String, from_client: &String) -> bool;
    fn encrypted(&self, password: &String) -> String;
    fn generate_token(&self, username: &String) -> Result<Token, String>;
    fn decode_token(&self) -> Result<String, String>;
    fn unvalidate_token(&self) -> Result<String, String>;
}
