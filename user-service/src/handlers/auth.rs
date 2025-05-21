use crate::{
    AppState,
    application::usecases::{base_usecase::BaseUsecase, create_usecase, signin_usecase},
    common::result::JSONResult,
    domains::models::user::{CheckUser, CreateUser},
};
use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

/// This is an asynchronous handler function for the "/signin" endpoint.
/// It takes two extracted arguments from the request:
/// - `Extension(state)`: shared application state (e.g., repositories, services)
/// - `Json(payload)`: the incoming JSON body, deserialized into a CheckUser struct
pub async fn signin(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CheckUser>,
) -> impl IntoResponse {
    let uc = signin_usecase::SignInUseCase::new(state.user_repo, state.auth_service);
    match uc.execute(&payload).await {
        Ok(data) => (StatusCode::OK, Json(data)).into_response(),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(JSONResult::new("error".to_owned(), "Echec de l'authentification".to_owned())),
        )
            .into_response(),
    }
}

/// This is an asynchronous handler function for the "/signin" endpoint.
/// It takes two extracted arguments from the request:
/// - `Extension(state)`: shared application state (e.g., repositories, services)
/// - `Json(payload)`: the incoming JSON body, deserialized into a CreateUser struct
pub async fn create(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let uc = create_usecase::CreateUseCase::new(state.user_repo);
    match uc.execute(&payload).await {
        Ok(_) => {
            (StatusCode::OK, Json(JSONResult::new("success".to_owned(), "success".to_string())))
                .into_response()
        }
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(JSONResult::new("error".to_owned(), "failed".to_string())),
        )
            .into_response(),
    }
}
