mod handlers;
mod routes;
use routes::all_routes;

#[tokio::main]
async fn main() {
    let app = all_routes::all_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
