use async_trait::async_trait;
use crate::domain::entities::auth::user::User;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError>;
}
