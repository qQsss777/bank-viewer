use crate::domain::{entities::user::User, value_objects::user::CheckUser};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: &mut User) -> Result<(), String>;
    async fn check_user(&self, user: &CheckUser) -> Result<(), String>;
}
