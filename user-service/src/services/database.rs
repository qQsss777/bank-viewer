use tokio_postgres::{Error, NoTls, Row};

use crate::models::user::{CheckUser, CreateUser};
pub struct DatabaseService {}
impl DatabaseService {
    pub async fn create_user(
        conn: &String,
        table: &String,
        user_infos: &CreateUser,
    ) -> Result<Vec<Row>, Error> {
        let (client, connection) = tokio_postgres::connect(conn, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let mut query = String::from("INSERT into ");
        query.push_str(table);
        query.push_str(" (");
        query.push_str(&user_infos.get_keys());
        query.push_str(") ");
        query.push_str("VALUES (");
        query.push_str(&user_infos.get_values());
        query.push_str(" )");
        println!("{}", query.to_string());
        client.query(&query, &[]).await
    }

    pub async fn check_user(conn: &String, table: &String, user: &CheckUser) {
        let (client, connection) = tokio_postgres::connect(conn, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let row: Result<Option<Row>, Error> = client
            .query_opt("SELECT username FROM users WHERE username = $1", &[&user.username])
            .await;
    }
}
