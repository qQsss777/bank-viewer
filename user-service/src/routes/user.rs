use crate::handlers::user;
use axum::{Router, routing::post};
pub fn user_routes() -> Router {
    Router::new().route("/create-account", post(user::create))
}
