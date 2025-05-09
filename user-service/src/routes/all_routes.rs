use std::sync::Arc;

use crate::handlers::default_method::default_method;
use crate::handlers::default_route::default_fallback;
use crate::models::database::Database;
use crate::routes::auth::auth_routes;
use crate::routes::user::user_routes;
use axum::{Extension, Router};

pub fn all_routes(state: Arc<Database>) -> Router {
    Router::new()
        .nest("/user", user_routes())
        .nest("/auth", auth_routes())
        .layer(Extension(state)) //
        .fallback(default_fallback)
        .method_not_allowed_fallback(default_method)
}
