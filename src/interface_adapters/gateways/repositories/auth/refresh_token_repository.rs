use async_trait::async_trait;
use crate::domain::entities::auth::refresh_token::RefreshToken;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn save(&self, token: &RefreshToken) -> Result<(), RepositoryError>;
    async fn find_by_token_hash(&self, hash: &str) -> Result<Option<RefreshToken>, RepositoryError>;
    async fn delete_by_token_hash(&self, hash: &str) -> Result<(), RepositoryError>;
    async fn update_last_used(&self, hash: &str) -> Result<(), RepositoryError>;
}
