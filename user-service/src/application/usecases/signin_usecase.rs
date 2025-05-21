use crate::domains::models::token::Token;
use crate::domains::{
    models::user::CheckUser, repositories::user_repository::UserRepository,
    services::jwt::JWTService,
};
use async_trait::async_trait;
use oul_bank_macro::New;
use std::sync::Arc;

use super::base_usecase::BaseUsecase;

#[derive(New)]
pub struct SignInUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
    jwt_service: Arc<dyn JWTService + Send + Sync>,
}

#[async_trait]
impl BaseUsecase<CheckUser, Result<Token, String>> for SignInUseCase {
    async fn execute(&self, payload: &CheckUser) -> Result<Token, String> {
        let pwd = self.repository.check_user(payload).await?;
        println!("{}, {}", pwd, payload.password);
        if pwd == payload.password {
            let token = self.jwt_service.generate_token(&payload.username)?;
            Ok(token)
        } else {
            Err("Echec".to_string())
        }
    }
}
