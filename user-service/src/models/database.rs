use oul_bank_macro::New;

#[derive(Clone, New)]
pub struct Database {
    host: String,
    table: String,
    database: String,
    username: String,
    password: String,
}

impl Database {
    pub fn get_connection_string(&self) -> String {
        format!(
            "host={} dbname={} user={} password={}",
            self.host, self.database, self.username, self.password
        )
    }

    pub fn get_table(&self) -> String {
        format!("{}.{}", self.database, self.table)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = Database::new(
            "Hello".to_owned(),
            "Hi".to_owned(),
            "Bonjour".to_owned(),
            "Ola".to_owned(),
            "Salut".to_owned(),
        );
        assert_eq!(result.get_table(), "Bonjour.Hi");
    }
}
