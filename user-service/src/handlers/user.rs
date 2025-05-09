use std::sync::Arc;

use crate::models::{database::Database, error::JSONError, user::CreateUser};
use axum::{Extension, extract::Json, response::IntoResponse};
use tokio_postgres::NoTls;

pub async fn create(
    Extension(state): Extension<Arc<Database>>,
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let db_string = state.get_connection_string();
    let (client, connection) = tokio_postgres::connect(&db_string, NoTls).await.unwrap();
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    //@TODO créer une macro

    let mut query = String::from("INSERT into ");
    query.push_str(&state.get_table());
    query.push_str(" (");
    query.push_str(&payload.get_keys());
    query.push_str(") ");
    query.push_str("VALUES (");
    query.push_str(&&payload.get_values());
    query.push_str(" )");
    println!("{}", query.to_string());
    match client.query(&query, &[]).await {
        Ok(_) => Json(JSONError::new(1, "Utilisateur créé")),
        Err(_) => Json(JSONError::new(0, "Echec de la création")),
    }
}

//INSERT INTO cars (brand, model, year)
//VALUES ('Ford', 'Mustang', 1964);
