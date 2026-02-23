use std::sync::Arc;

use crate::framework::{
    controllers::{default_method::default_method, default_route::default_fallback},
    middlewares::cors::cors_middleware,
    routes::auth::auth_routes,
    state::state::AppState,
};
use axum::{Extension, Router, middleware};

pub fn all_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/auth", auth_routes())
        .fallback(default_fallback)
        .method_not_allowed_fallback(default_method)
        .layer(middleware::from_fn(cors_middleware))
        .layer(Extension(state))
}
