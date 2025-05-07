use axum::response::{Html, IntoResponse};

pub async fn default_method() -> impl IntoResponse {
    Html("<h1>Méthode non autorisée</h1>")
}
