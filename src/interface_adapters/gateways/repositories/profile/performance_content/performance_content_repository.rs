use async_trait::async_trait;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait PerformanceContentRepository: Send + Sync {
    async fn upload_content(
        &self,
        profile_id: &str,
        performance_id: &str,
        content: &str,
    ) -> Result<String, RepositoryError>;

    async fn get_content(
        &self,
        profile_id: &str,
        performance_id: &str,
    ) -> Result<String, RepositoryError>;

    async fn delete_content(
        &self,
        profile_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError>;
}
