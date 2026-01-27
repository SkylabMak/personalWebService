use async_trait::async_trait;

use crate::domain::entities::application::feature_status::app_feature_status::AppFeatureStatus;
use crate::interface_adapters::gateways::common::{
    repository_error::RepositoryError,
};

#[async_trait]
pub trait AppRepository: Send + Sync {
    async fn find_feature_statuses_by_website_id(
        &self,
        website_id: &str,
    ) -> Result<Vec<AppFeatureStatus>, RepositoryError>;
}
