use std::sync::Arc;

use crate::domains::repositories::user_repository::UserRepository;
use crate::handlers::default_method::default_method;
use crate::handlers::default_route::default_fallback;
use crate::routes::auth::auth_routes;
use axum::{Extension, Router};

pub fn all_routes(state: Arc<dyn UserRepository>) -> Router {
    Router::new()
        .nest("/auth", auth_routes())
        .layer(Extension(state)) //
        .fallback(default_fallback)
        .method_not_allowed_fallback(default_method)
}
