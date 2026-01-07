use async_trait::async_trait;

use crate::domain::entities::website::feature_status::website_feature_status::WebsiteFeatureStatus;
use crate::interface_adapters::gateways::common::{
    repository_error::RepositoryError,
};

#[async_trait]
pub trait WebsiteRepository: Send + Sync {
    async fn find_feature_statuses_by_website_id(
        &self,
        website_id: &str,
    ) -> Result<Vec<WebsiteFeatureStatus>, RepositoryError>;
}
