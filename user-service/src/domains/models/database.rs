use oul_bank_macro::New;

#[derive(Clone, New)]
pub struct Database {
    host: String,
    schema: String,
    database: String,
    table: String,
    username: String,
    password: String,
    port: String,
}

impl Database {
    pub fn get_connection_string(&self) -> String {
        format!(
            "host={} port={} dbname={} user={} password={}",
            self.host, self.port, self.database, self.username, self.password
        )
    }

    pub fn get_table(&self) -> String {
        format!("{}.{}", self.schema, self.table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = Database::new(
            "Hello".to_owned(),
            "Schema".to_owned(),
            "Hi".to_owned(),
            "Bonjour".to_owned(),
            "Ola".to_owned(),
            "Salut".to_owned(),
            "5433".to_owned(),
        );
        assert_eq!(result.get_table(), "Bonjour.Hi");
    }
}
