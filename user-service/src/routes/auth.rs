use axum::{Router, routing::post};

use crate::handlers::auth::generate_token;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/generateToken", post(generate_token))
        .route("/revokeToken", post(|| async { "Hello, World!" }))
        .route("/validateToken", post(|| async { "Hello, World!" }))
}
