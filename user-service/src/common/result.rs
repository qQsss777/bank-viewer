use oul_bank_macro::New;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, New)]
pub struct JSONResult {
    code: i32,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let err: JSONResult = JSONResult::new(1, "Error".to_owned());
        assert_eq!(err.message, "Error");
    }
}
