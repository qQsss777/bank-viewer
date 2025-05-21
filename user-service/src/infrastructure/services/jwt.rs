use crate::domains::models::token::Token;
use crate::domains::services::jwt::JWTService;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use oul_bank_macro::New;
use sha2::Sha384;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(New)]
pub struct JWTServiceImpl {
    secret: String,
}

impl JWTService for JWTServiceImpl {
    fn check_password(&self, from_db: &String, from_client: &String) -> bool {
        true
    }

    fn encrypted(&self, password: &String) -> String {
        String::new()
    }

    fn generate_token(&self, username: &String) -> Result<Token, String> {
        let key: Hmac<Sha384> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|e| e.to_string())?;
        println!("{}", self.secret);
        let mut claims = BTreeMap::new();
        claims.insert("sub", username);
        let iat: u128 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let binding = iat.to_string();
        claims.insert("iat", &binding);
        let token_str = claims.sign_with_key(&key).map_err(|e| e.to_string())?;
        Ok(Token::new(token_str, iat))
    }

    fn decode_token(&self) -> Result<String, String> {
        Ok(String::new())
    }

    fn unvalidate_token(&self) -> Result<String, String> {
        Ok(String::new())
    }
}
