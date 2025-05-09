use axum::{http::StatusCode, response::IntoResponse};
pub async fn default_method() -> impl IntoResponse {
    StatusCode::METHOD_NOT_ALLOWED
}
