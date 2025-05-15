pub trait JWTService {
    fn check_password(from_db: &String, from_client: &String) -> bool;
    fn encrypted(password: &String) -> String;
    fn generate_token() -> String;
    fn decode_token() -> String;
    fn unvalidate_token() -> String;
}
