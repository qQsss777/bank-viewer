use std::sync::Arc;

use argon2::password_hash::SaltString;
use base64::{Engine as _, engine::general_purpose};
use rand::RngCore;
use rand::rngs::OsRng;

use crate::{
    domain::value_objects::{database::Database, error::PasswordHashError},
    infrastructure::{
        repositories::user::UserRepositoryPostgres,
        services::{hash_password::HashPasswordServiceImpl, jwt::JWTServiceImpl},
    },
    tools::random::generate_random_string,
};

#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<UserRepositoryPostgres>,
    pub auth_service: Arc<JWTServiceImpl>,
}

pub async fn create_state() -> Arc<AppState> {
    let db = Database::new(
        std::env::var("PG_HOST").unwrap(),
        std::env::var("PG_SCHEMA").unwrap(),
        std::env::var("PG_DATABASE").unwrap(),
        std::env::var("PG_TABLE").unwrap(),
        std::env::var("PG_USER").unwrap(),
        std::env::var("PG_PASSWORD").unwrap(),
        std::env::var("PG_PORT").unwrap(),
    );
    let mut salt_bytes = [0u8; 16];
    let mut rng = OsRng;
    rng.fill_bytes(&mut salt_bytes);
    let salt =
        SaltString::encode_b64(&salt_bytes).map_err(|_| PasswordHashError::HashFailed).unwrap();
    let password_hasher = Arc::new(HashPasswordServiceImpl { salt });
    let user_repo = Arc::new(UserRepositoryPostgres::new(password_hasher, db));
    let secret_key = generate_random_string(32);
    let auth_service = Arc::new(JWTServiceImpl::new(secret_key.to_owned()));
    Arc::new(AppState { user_repo, auth_service })
}
