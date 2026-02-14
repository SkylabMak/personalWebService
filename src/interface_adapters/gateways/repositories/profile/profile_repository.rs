use async_trait::async_trait;
use crate::domain::entities::profile::profile::Profile;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Option<Profile>, RepositoryError>;
}
