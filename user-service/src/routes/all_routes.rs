use crate::handlers::default_method::default_method;
use crate::handlers::default_route::default_fallback;
use crate::routes::auth;
use axum::Router;
pub fn all_routes() -> Router {
    Router::new()
        .merge(auth::user_routes())
        .fallback(default_fallback)
        .method_not_allowed_fallback(default_method)
}
