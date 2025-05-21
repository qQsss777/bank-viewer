use axum::{Router, routing::post};

use crate::handlers::auth::{create, signin};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signin", post(signin))
        .route("/create-account", post(create))
        .route("/logout", post(|| async { "Hello, World!" }))
        .route("/validateToken", post(|| async { "Hello, World!" }))
}
