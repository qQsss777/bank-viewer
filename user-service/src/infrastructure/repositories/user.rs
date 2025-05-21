use async_trait::async_trait;
use oul_bank_macro::New;
use tokio_postgres::NoTls;

use crate::domains::{
    models::{
        database::Database,
        user::{CheckUser, CreateUser},
    },
    repositories::user_repository::UserRepository,
};
#[derive(New)]
pub struct UserRepositoryPostgres {
    pub database: Database,
}
#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn create_user(&self, user_infos: &CreateUser) -> Result<(), String> {
        let conn = self.database.get_connection_string();
        let user_table = self.database.get_table();
        let (client, connection) =
            tokio_postgres::connect(&conn, NoTls).await.map_err(|e| e.to_string())?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let mut query = String::from("INSERT into ");
        query.push_str(&user_table);
        query.push_str(" (");
        query.push_str(&user_infos.get_keys());
        query.push_str(") ");
        query.push_str("VALUES (");
        query.push_str(&user_infos.get_values());
        query.push_str(" )");
        client.query(&query, &[]).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn check_user(&self, user: &CheckUser) -> Result<String, String> {
        let conn = self.database.get_connection_string();
        let user_table = self.database.get_table();
        let (client, connection) =
            tokio_postgres::connect(&conn, NoTls).await.map_err(|e| e.to_string())?;
        tokio::spawn(async move {
            if let Err(e) = connection.await.map_err(|e| e.to_string()) {
                eprintln!("connection error: {}", e);
            }
        });
        let query = format!("SELECT username, password FROM {} WHERE username = $1", user_table);
        let row = client.query_one(&query, &[&user.username]).await.map_err(|e| e.to_string())?;
        Ok(row.get(1))
    }
}
