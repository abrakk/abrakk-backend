use async_trait::async_trait;
use uuid::Uuid;

use crate::entities::user::User;
use crate::errors::DomainError;

/// Repository trait for User persistence.
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<User, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<User, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<User, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
    async fn update(&self, user: User) -> Result<User, DomainError>;
}
