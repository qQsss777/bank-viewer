use std::sync::Arc;

use crate::{
    application::dto::user_dto::CreateUserDTO,
    domain::{entities::user::User, repositories::user_repository::UserRepository},
};
use async_trait::async_trait;
use oul_bank_macro::New;

use super::base_usecase::BaseUsecase;

#[derive(New)]
pub struct CreateUseCase {
    repository: Arc<dyn UserRepository + Send + Sync>,
}

#[async_trait]
impl BaseUsecase<CreateUserDTO, Result<(), String>> for CreateUseCase {
    async fn execute(&self, payload: &CreateUserDTO) -> Result<(), String> {
        let mut user = User::try_from(payload).map_err(|e| e.to_string())?;
        self.repository.create_user(&mut user).await
    }
}
