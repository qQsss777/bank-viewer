mod application;
mod common;
mod domain;
mod framework;
mod infrastructure;
mod tools;
use crate::framework::{server::server::create_server, state::state::create_state};
use dotenv::dotenv;
use tokio::signal;

#[tokio::main]
async fn main() {
    //get environment variables
    dotenv().ok();
    let state = create_state().await;
    let app = create_server(state).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1003").await.unwrap();
    axum::serve(listener, app).with_graceful_shutdown(signal()).await.unwrap();
}

async fn signal() {
    // Capture le Ctrl+C
    signal::ctrl_c().await.expect("Erreur lors de l'écoute du signal Ctrl+C");
    println!("\nSignal reçu : arrêt en cours…");
}
