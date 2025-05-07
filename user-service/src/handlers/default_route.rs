use axum::response::{Html, IntoResponse};

pub async fn default_fallback() -> impl IntoResponse {
    Html("<h1>Route non implémentée</h1>")
}
