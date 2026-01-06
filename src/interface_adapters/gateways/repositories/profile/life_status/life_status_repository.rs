use async_trait::async_trait;

use crate::interface_adapters::gateways::common::{
    repository_error::RepositoryError,
};

pub struct LifeStatusData {
    pub name: String,
    pub description: Option<String>,
    pub color_token: String,
}

#[async_trait]
pub trait LifeStatusRepository: Send + Sync {
    async fn find_current_by_profile_id(
        &self,
        profile_id: &str,
    ) -> Result<Option<LifeStatusData>, RepositoryError>;
}
