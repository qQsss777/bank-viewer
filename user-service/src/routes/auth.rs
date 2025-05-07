use axum::{Router, routing::get};

pub fn user_routes() -> Router {
    Router::new()
        .route("/generateToken", get(|| async { "Hello, World!" }))
        .route("/revokeToken", get(|| async { "Hello, World!" }))
        .route("/validateToken", get(|| async { "Hello, World!" }))
}
