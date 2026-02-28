use axum::{Router, routing::post};

use crate::framework::controllers::auth::{create, signin, validate_token, who_is};
pub fn auth_routes() -> Router {
    Router::new()
        .route("/signin", post(signin))
        .route("/create-account", post(create))
        .route("/logout", post(|| async { "Hello, World!" }))
        .route("/validate", post(validate_token))
        .route("/whois", post(who_is))
}
