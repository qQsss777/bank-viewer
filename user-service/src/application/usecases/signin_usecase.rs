use std::sync::Arc;

use crate::{
    common::result::JSONResult,
    domains::{
        models::user::CheckUser, repositories::user_repository::UserRepository,
        services::jwt::JWTService,
    },
};
use async_trait::async_trait;
use oul_bank_macro::New;

use super::base_usecase::BaseUsecase;

#[derive(New)]
pub struct SignInUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
    jwt_service: Arc<dyn JWTService + Send + Sync>,
}

#[async_trait]
impl BaseUsecase<CheckUser, Result<String, String>> for SignInUseCase {
    async fn execute(&self, payload: &CheckUser) -> Result<String, String> {
        let pwd = self.repository.check_user(payload).await?;
        println!("{}", pwd);
        Ok("e".to_string())
    }
}
