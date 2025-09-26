use axum::{Router, routing::post};

use crate::application::handlers::auth::{create, signin, validate_token};

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signin", post(signin))
        .route("/create-account", post(create))
        .route("/logout", post(|| async { "Hello, World!" }))
        .route("/validate", post(validate_token))
}
