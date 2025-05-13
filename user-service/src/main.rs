mod handlers;
mod models;
mod routes;
mod services;
use std::sync::Arc;

use dotenv::dotenv;
use models::database::Database;
use routes::all_routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = Database::new(
        std::env::var("PG_HOST").unwrap(),
        std::env::var("PG_DATABASE").unwrap(),
        std::env::var("PG_TABLE").unwrap(),
        std::env::var("PG_USER").unwrap(),
        std::env::var("PG_PASSWORD").unwrap(),
    );
    let shared_state = Arc::new(db);
    let app = all_routes::all_routes(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
