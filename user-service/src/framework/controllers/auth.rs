use std::sync::Arc;

use crate::{
    application::{
        dto::user_dto::CreateUserDTO,
        usecases::{base_usecase::BaseUsecase, create_usecase, signin_usecase, validate_usecase},
    },
    common::result::JSONResult,
    domain::value_objects::{token::TokenToValidate, user::CheckUser},
    framework::state::state::AppState,
};
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

/// This is an asynchronous handler function for the "/signin" endpoint.
/// It takes two extracted arguments from the request:
/// - `Extension(state)`: shared application state (e.g., repositories, services)
/// - `Json(payload)`: the incoming JSON body, deserialized into a CheckUser struct
pub async fn signin(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CheckUser>,
) -> impl IntoResponse {
    let uc: signin_usecase::SignInUseCase =
        signin_usecase::SignInUseCase::new(state.user_repo.clone(), state.auth_service.clone());
    match uc.execute(&payload).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(JSONResult::new("error".to_owned(), "Authentification failed".to_owned())),
        )
            .into_response(),
    }
}

/// This is an asynchronous handler function for the "/create-account" endpoint.
/// It takes two extracted arguments from the request:
/// - `Extension(state)`: shared application state (e.g., repositories, services)
/// - `Json(payload)`: the incoming JSON body, deserialized into a User struct
pub async fn create(
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<CreateUserDTO>,
) -> impl IntoResponse {
    let uc: create_usecase::CreateUseCase =
        create_usecase::CreateUseCase::new(state.user_repo.clone());
    match uc.execute(&payload).await {
        Ok(_) => {
            (StatusCode::OK, Json(JSONResult::new("success".to_owned(), "success".to_string())))
                .into_response()
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(JSONResult::new("error".to_owned(), "failed to create user".to_string())),
        )
            .into_response(),
    }
}

/// This is a synchronous handler function for the "/valiodate" endpoint.
/// It takes two extracted arguments from the request:
/// - `Extension(state)`: shared application state (e.g., repositories, services)
/// - `Json(payload)`: the incoming JSON body, deserialized into a Token struct
pub async fn validate_token(
    Extension(state): Extension<AppState>,
    Json(payload): Json<TokenToValidate>,
) -> impl IntoResponse {
    let uc: validate_usecase::ValidateTokenUsecase =
        validate_usecase::ValidateTokenUsecase::new(state.auth_service);
    match uc.execute_sync(&payload) {
        Ok(_) => {
            (StatusCode::OK, Json(JSONResult::new("success".to_owned(), "token valid".to_string())))
                .into_response()
        }
        Err(e) => {
            (StatusCode::OK, Json(JSONResult::new("error".to_owned(), "token invalid".to_string())))
                .into_response()
        }
    }
}
