use std::collections::BTreeMap;

use crate::domain::value_objects::token::Token;
use async_trait::async_trait;

#[async_trait]
pub trait TokenService: Send + Sync {
    fn generate_token(&self, username: &String) -> Result<Token, String>;
    fn decode_token(&self, token: &String) -> Result<BTreeMap<String, String>, String>;
    fn validate_token(&self, token: &String) -> Result<String, String>;
}
