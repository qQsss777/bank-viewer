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

    fn decode_token(&self, token_str: &String) -> Result<BTreeMap<String, String>, String> {
        let key: Hmac<Sha384> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|e| e.to_string())?;
        let token_informations: jwt::Token<Header, BTreeMap<String, String>, _> =
            token_str.verify_with_key(&key).map_err(|e| e.to_string())?;
        Ok(token_informations.claims().clone())
    }

    fn validate_token(&self, token_str: &String) -> Result<String, String> {
        let key: Hmac<Sha384> =
            Hmac::new_from_slice(self.secret.as_bytes()).map_err(|e| e.to_string())?;
        let _token: jwt::Token<Header, BTreeMap<String, String>, _> =
            token_str.verify_with_key(&key).map_err(|e| e.to_string())?;
        Ok("success".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token() {
        let jwt_service = JWTServiceImpl { secret: "hello".to_string() };
        let username = "vegeta".to_string();
        let token = jwt_service.generate_token(&username).unwrap();
        let token_str = token.token;
        assert_eq!(token_str.starts_with("eyJhbGciOiJIUzM4NCJ9"), true);
    }
    #[test]
    fn validate_token() {
        let jwt_service = JWTServiceImpl { secret: "hello".to_string() };
        let token: String = "eyJhbGciOiJIUzM4NCJ9.eyJpYXQiOiIxNzcyMjEzMTc2ODk2Iiwic3ViIjoidmVnZXRhIn0.znzE-HEAgC63XTXeMN861AM-8MEJ1RsDBSdlP31INGY6EU3zXDLDE4Lb9eczsGuX".to_string();
        let token_validated = jwt_service.validate_token(&token).unwrap();
        assert_eq!(token_validated, "success".to_string());
    }
    #[test]
    fn decode_token() {
        let jwt_service = JWTServiceImpl { secret: "hello".to_string() };
        let token = "eyJhbGciOiJIUzM4NCJ9.eyJpYXQiOiIxNzcyMjEzMTc2ODk2Iiwic3ViIjoidmVnZXRhIn0.znzE-HEAgC63XTXeMN861AM-8MEJ1RsDBSdlP31INGY6EU3zXDLDE4Lb9eczsGuX".to_string();
        let token_infos = jwt_service.decode_token(&token).unwrap();
        let username = &token_infos["sub"];
        assert_eq!(username, "vegeta");
    }
}
