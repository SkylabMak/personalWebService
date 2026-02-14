use async_trait::async_trait;
use crate::domain::entities::profile::performance::performance::Performance;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait PerformanceRepository: Send + Sync {
    async fn create(&self, performance: Performance) -> Result<Performance, RepositoryError>;
    async fn update(&self, performance: Performance) -> Result<Performance, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Performance>, RepositoryError>;
    async fn find_by_profile_id(
        &self,
        profile_id: &str,
        visibility_id: Option<&str>,
    ) -> Result<Vec<Performance>, RepositoryError>;
    async fn delete(&self, id: &str) -> Result<(), RepositoryError>;
    
    // Image usage tracking
    async fn track_image_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError>;
    async fn untrack_image_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError>;
    async fn get_tracked_images(&self, performance_id: &str) -> Result<Vec<String>, RepositoryError>;
    async fn delete_image_usage_by_performance_id(&self, performance_id: &str) -> Result<(), RepositoryError>;
}
