use async_trait::async_trait;

use crate::domain::entities::profile::life_status::life_status::LifeStatus;
use crate::interface_adapters::gateways::common::{
    repository_error::RepositoryError,
};

#[async_trait]
pub trait LifeStatusRepository: Send + Sync {
    async fn find_current_by_profile_id(
        &self,
        profile_id: &str,
    ) -> Result<Option<LifeStatus>, RepositoryError>;
}
