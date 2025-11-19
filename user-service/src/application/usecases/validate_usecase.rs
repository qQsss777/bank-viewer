use crate::application::usecases::base_usecase::BaseUsecase;
use crate::domains::services::jwt::JWTService;
use crate::domains::value_objects::token::TokenToValidate;
use oul_bank_macro::New;
use std::sync::Arc;

#[derive(New)]
pub struct ValidateTokenUsecase {
    jwt_service: Arc<dyn JWTService + Send + Sync>,
}

impl BaseUsecase<TokenToValidate, Result<String, String>> for ValidateTokenUsecase {
    fn execute_sync(&self, payload: &TokenToValidate) -> Result<String, String> {
        self.jwt_service.validate_token(&payload.token)
    }
}
