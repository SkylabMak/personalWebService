use async_trait::async_trait;
use crate::domain::entities::profile::announce::announce::Announce;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait AnnounceRepository: Send + Sync {
    async fn find_active_by_profile_id(
        &self,
        profile_id: &str,
    ) -> Result<Vec<Announce>, RepositoryError>;
}
