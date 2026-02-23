use crate::application::services::token::TokenService;
use crate::domain::value_objects::token::Token;
use crate::domain::{
    repositories::user_repository::UserRepository, value_objects::user::CheckUser,
};
use async_trait::async_trait;
use oul_bank_macro::New;
use std::sync::Arc;

use super::base_usecase::BaseUsecase;

#[derive(New)]
pub struct SignInUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
    token_service: Arc<dyn TokenService + Send + Sync>,
}

#[async_trait]
impl BaseUsecase<CheckUser, Result<Token, String>> for SignInUseCase {
    async fn execute(&self, payload: &CheckUser) -> Result<Token, String> {
        self.repository.check_user(payload).await?;
        self.token_service.generate_token(&payload.username)
    }
}
