use crate::{
    AppState,
    application::handlers::{default_method::default_method, default_route::default_fallback},
    application::routes::auth::auth_routes,
};
use axum::{Extension, Router};

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        .nest("/auth", auth_routes())
        .layer(Extension(state)) //
        .fallback(default_fallback)
        .method_not_allowed_fallback(default_method)
}
