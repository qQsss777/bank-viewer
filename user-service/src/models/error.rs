use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JSONError {
    code: i32,
    message: String,
}

impl JSONError {
    pub fn new(code: i32, message: &str) -> Self {
        JSONError { code: code, message: message.to_string() }
    }
}
