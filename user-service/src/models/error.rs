use oul_bank_macro::New;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, New)]
pub struct JSONError {
    code: i32,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let err: JSONError = JSONError::new(1, "Error".to_owned());
        assert_eq!(err.message, "Error");
    }
}
