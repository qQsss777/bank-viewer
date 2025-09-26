use axum::{http::StatusCode, response::IntoResponse};
pub async fn default_fallback() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}
