use async_trait::async_trait;
use crate::domain::entities::profile::social::social::Social;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait SocialRepository: Send + Sync {
    async fn find_by_profile_id(&self, profile_id: &str) -> Result<Vec<Social>, RepositoryError>;
}
