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
    async fn sync_image_usage(
        &self,
        performance_id: &str,
        current_image_ids: &[String],
    ) -> Result<(), RepositoryError>;
    async fn delete_image_usage_by_performance_id(&self, performance_id: &str) -> Result<(), RepositoryError>;
    async fn find_images_by_performance_id(
        &self,
        performance_id: &str,
    ) -> Result<Vec<crate::domain::entities::profile::image::image::Image>, RepositoryError>;
}
