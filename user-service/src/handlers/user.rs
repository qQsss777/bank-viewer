use std::sync::Arc;

use crate::{
    models::{database::Database, error::JSONError, user::CreateUser},
    services::database::DatabaseService,
};
use axum::{Extension, extract::Json, response::IntoResponse};
use tokio_postgres::NoTls;

pub async fn create(
    Extension(state): Extension<Arc<Database>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let db_string = state.get_connection_string();
    let table = state.get_table();
    match DatabaseService::create_user(&db_string, &table, &payload).await {
        Ok(_) => Json(JSONError::new(1, "Utilisateur créé".to_owned())),
        Err(_) => Json(JSONError::new(0, "Echec de la création".to_owned())),
    }
}
