use axum::Router;

use crate::framework::{routes::all_routes::all_routes, state::state::AppState};

pub async fn create_server(shared_state: AppState) -> Router {
    println!("\noks…");
    all_routes(shared_state)
}
