use std::sync::Arc;

use axum::{Extension, Json, http::StatusCode, response::IntoResponse};

use crate::models::{database::Database, user::CheckUser};

pub fn generate_token(
    Extension(state): Extension<Arc<Database>>,
    Json(payload): Json<CheckUser>,
) -> impl IntoResponse {
    StatusCode::MOVED_PERMANENTLY
}
