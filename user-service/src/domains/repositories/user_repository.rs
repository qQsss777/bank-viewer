use crate::domains::models::user::{CheckUser, CreateUser};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user_infos: &CreateUser) -> Result<(), String>;
    async fn check_user(&self, user: &CheckUser) -> Result<String, String>;
}
