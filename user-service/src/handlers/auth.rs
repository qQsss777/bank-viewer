use crate::{
    AppState,
    application::usecases::{base_usecase::BaseUsecase, create_usecase, signin_usecase},
    common::result::JSONResult,
    domains::models::user::{CheckUser, CreateUser},
};
use axum::{Extension, Json, response::IntoResponse};

pub async fn signin(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CheckUser>,
) -> impl IntoResponse {
    let uc = signin_usecase::SignInUseCase::new(state.user_repo, state.auth_service);
    match uc.execute(&payload).await {
        Ok(data) => Json(JSONResult::new(0, data)),
        Err(_) => Json(JSONResult::new(0, "Echec de l'authentification".to_owned())),
    }
}

pub async fn create(
    Extension(state): Extension<AppState>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let uc = create_usecase::CreateUseCase::new(state.user_repo);
    match uc.execute(&payload).await {
        Ok(_) => Json(JSONResult::new(0, "success".to_string())),
        Err(_) => Json(JSONResult::new(0, "failed".to_string())),
    }
}
