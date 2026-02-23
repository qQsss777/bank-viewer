use crate::application::services::token::TokenService;
use crate::domain::value_objects::token::Token;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, VerifyWithKey};
use oul_bank_macro::New;
use sha2::Sha384;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(New)]
pub struct JWTServiceImpl {
    secret: String,
}

impl TokenService for JWTServiceImpl {
    fn generate_token(&self, username: &String) -> Result<Token, String> {
        let key: Hmac<Sha384> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|e| e.to_string())?;
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

    fn validate_token(&self, token_str: &String) -> Result<String, String> {
        let key: Hmac<Sha384> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|e| e.to_string())?;
        let _token: jwt::Token<Header, BTreeMap<String, String>, _> =
            token_str.verify_with_key(&key).map_err(|e| e.to_string())?;
        Ok("success".to_string())
    }
}
