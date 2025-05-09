use axum::{http::StatusCode, response::IntoResponse};

pub async fn generate_token() -> impl IntoResponse {
    StatusCode::MOVED_PERMANENTLY
}
