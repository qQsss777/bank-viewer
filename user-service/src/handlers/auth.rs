use std::sync::Arc;

use axum::{Extension, Json, response::IntoResponse};

use crate::{
    application::usecases::{base_usecase::BaseUsecase, create_usecase},
    common::result::JSONResult,
    domains::{
        models::user::{CheckUser, CreateUser},
        repositories::user_repository::UserRepository,
    },
};

pub async fn signin(
    Extension(state): Extension<Arc<dyn UserRepository>>,
    Json(payload): Json<CheckUser>,
) -> impl IntoResponse {
    Json(JSONResult::new(0, "Echec de l'authentification".to_owned()))
}

pub async fn create(
    Extension(state): Extension<Arc<dyn UserRepository>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let uc = create_usecase::CreateUseCase::new(state);
    match uc.execute(&payload).await {
        Ok(_) => Json(JSONResult::new(0, "success".to_string())),
        Err(_) => Json(JSONResult::new(0, "failed".to_string())),
    }
}
