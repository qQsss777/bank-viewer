use std::sync::Arc;

use crate::{
    common::result::JSONResult,
    domains::{models::user::CreateUser, repositories::user_repository::UserRepository},
};
use async_trait::async_trait;
use oul_bank_macro::New;

use super::base_usecase::BaseUsecase;

#[derive(New)]
pub struct CreateUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

#[async_trait]
impl BaseUsecase<CreateUser, Result<(), String>> for CreateUseCase {
    async fn execute(&self, payload: &CreateUser) -> Result<(), String> {
        self.repository.create_user(payload).await
    }
}
