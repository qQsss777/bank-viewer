use std::sync::Arc;

use async_trait::async_trait;
use oul_bank_macro::New;
use tokio_postgres::NoTls;

use crate::domain::{
    entities::user::User,
    repositories::user_repository::UserRepository,
    services::hash_password::HashPasswordService,
    value_objects::{database::Database, user::CheckUser},
};
#[derive(New)]
pub struct UserRepositoryPostgres {
    pub password_hasher: Arc<dyn HashPasswordService + Send + Sync>,
    pub database: Database,
}

#[async_trait]
impl UserRepository for UserRepositoryPostgres {
    async fn create_user(&self, user: &mut User) -> Result<(), String> {
        let conn = self.database.get_connection_string();
        let user_table = self.database.get_table();
        let (client, connection) =
            tokio_postgres::connect(&conn, NoTls).await.map_err(|e| e.to_string())?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let password_hashed =
            self.password_hasher.hash_password(&user.password).map_err(|e| e.to_string())?;
        user.change_password(password_hashed);
        let mut query = String::from("INSERT into ");
        query.push_str(&user_table);
        query.push_str(" (");
        query.push_str(&user.get_keys());
        query.push_str(") ");
        query.push_str("VALUES (");
        query.push_str(&user.get_values());
        query.push_str(" )");
        client.query(&query, &[]).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn check_user(&self, user: &CheckUser) -> Result<(), String> {
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
        let pwd: String = row.get(1);
        self.password_hasher.check_password(&pwd, &user.password)
    }
}
