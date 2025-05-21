use oul_bank_macro::New;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, New)]
pub struct Token {
    token: String,
    created_at: u128,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let token: Token = Token::new("ffrfr".to_owned(), 1747841406596);
        assert_eq!(token.token, "ffrfr");
    }
}
