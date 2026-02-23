use crate::application::services::token::TokenService;
use crate::application::usecases::base_usecase::BaseUsecase;
use crate::domain::value_objects::token::TokenToValidate;
use oul_bank_macro::New;
use std::sync::Arc;

#[derive(New)]
pub struct ValidateTokenUsecase {
    jwt_service: Arc<dyn TokenService + Send + Sync>,
}

impl BaseUsecase<TokenToValidate, Result<String, String>> for ValidateTokenUsecase {
    fn execute_sync(&self, payload: &TokenToValidate) -> Result<String, String> {
        self.jwt_service.validate_token(&payload.token)
    }
}
