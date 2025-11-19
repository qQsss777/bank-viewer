use oul_bank_macro::New;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, New)]
pub struct Token {
    token: String,
    created_at: u128,
}

#[derive(Deserialize, Serialize, New)]
pub struct TokenToValidate {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let token: Token = Token::new("ffrfr".to_owned(), 1747841406596);
        assert_eq!(token.token, "ffrfr");

        let token_validate: TokenToValidate = TokenToValidate::new("aaaa".to_owned());
        assert_eq!(token_validate.token, "aaaa")
    }
}
