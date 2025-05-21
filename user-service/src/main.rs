mod application;
mod common;
mod domains;
mod handlers;
mod infrastructure;
mod routes;
mod tools;
use crate::domains::models::database::Database;
use dotenv::dotenv;
use infrastructure::repositories::user::UserRepositoryPostgres;
use infrastructure::services::jwt::JWTServiceImpl;
use routes::all_routes;
use std::env;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    user_repo: Arc<UserRepositoryPostgres>,
    auth_service: Arc<JWTServiceImpl>,
}

#[tokio::main]
async fn main() {
    //get environment variables
    dotenv().ok();
    let db_host: String = env::var("PG_HOST").expect("PG_HOST must be set");
    let db_schema: String = env::var("PG_SCHEMA").expect("PG_SCHEMA must be set");
    let db_db = env::var("PG_DATABASE").expect("PG_DATABASE must be set");
    let db_table = env::var("PG_TABLE").expect("PG_TABLE must be set");
    let db_user = env::var("PG_USER").expect("PG_USER must be set");
    let db_password = env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
    let db_port = env::var("PG_PORT").expect("PG_PORT must be set");
    let db = Database::new(db_host, db_schema, db_db, db_table, db_user, db_password, db_port);

    //create repo and service for injection dependencies through state
    let user_repo = Arc::new(UserRepositoryPostgres::new(db));
    let secret_key = tools::random::generate_random_string(32);
    let auth_service = Arc::new(JWTServiceImpl::new(secret_key.to_owned()));
    let shared_state: AppState = AppState { user_repo, auth_service };

    // create and start app
    let app = all_routes::all_routes(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
