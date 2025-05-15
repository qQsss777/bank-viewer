use crate::domains::services::jwt::JWTService;

struct JWTServiceImpl;

impl JWTService for JWTServiceImpl {
    fn check_password(from_db: &String, from_client: &String) -> bool {
        true
    }

    fn encrypted(password: &String) -> String {
        String::new()
    }

    fn generate_token() -> String {
        String::new()
    }

    fn decode_token() -> String {
        String::new()
    }

    fn unvalidate_token() -> String {
        String::new()
    }
}
