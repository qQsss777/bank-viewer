use std::sync::Arc;

use crate::{
    domains::value_objects::database::Database,
    infrastructure::{repositories::user::UserRepositoryPostgres, services::jwt::JWTServiceImpl},
    tools::random::generate_random_string,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<UserRepositoryPostgres>,
    pub auth_service: Arc<JWTServiceImpl>,
}

pub async fn create_state() -> AppState {
    let db = Database::new(
        std::env::var("PG_HOST").unwrap(),
        std::env::var("PG_SCHEMA").unwrap(),
        std::env::var("PG_DATABASE").unwrap(),
        std::env::var("PG_TABLE").unwrap(),
        std::env::var("PG_USER").unwrap(),
        std::env::var("PG_PASSWORD").unwrap(),
        std::env::var("PG_PORT").unwrap(),
    );
    //create repo and service for injection dependencies through state
    let user_repo = Arc::new(UserRepositoryPostgres::new(db));
    let secret_key = generate_random_string(32);
    let auth_service = Arc::new(JWTServiceImpl::new(secret_key.to_owned(), Vec::new()));
    AppState { user_repo, auth_service }
}
