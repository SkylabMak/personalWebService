use async_trait::async_trait;
use crate::domain::entities::profile::image::image::Image;
use crate::domain::entities::profile::image::image_usage::ImageUsageInfo;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait ImageRepository: Send + Sync {
    async fn find_all_by_profile_id(
        &self,
        profile_id: &str,
        search: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<(Vec<(Image, i32)>, usize), RepositoryError>;

    async fn find_by_id_and_profile_id(
        &self,
        id: &str,
        profile_id: &str,
    ) -> Result<Option<(Image, i32)>, RepositoryError>;

    async fn find_usage_by_image_id(
        &self,
        image_id: &str,
        profile_id: &str,
    ) -> Result<Vec<ImageUsageInfo>, RepositoryError>;

    async fn create(&self, image: Image) -> Result<(), RepositoryError>;

    async fn track_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError>;

    async fn untrack_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError>;
    
    async fn update_metadata(
        &self,
        id: &str,
        profile_id: &str,
        alt_text: Option<String>,
        caption: Option<String>,
    ) -> Result<(), RepositoryError>;

    async fn delete(&self, id: &str, profile_id: &str) -> Result<(), RepositoryError>;

    async fn force_delete(&self, id: &str, profile_id: &str) -> Result<(), RepositoryError>;

    async fn find_unused_by_profile_id(
        &self,
        profile_id: &str,
        days_old: i32,
    ) -> Result<Vec<Image>, RepositoryError>;

    async fn delete_unused_by_profile_id(
        &self,
        profile_id: &str,
        days_old: i32,
    ) -> Result<(i64, i64), RepositoryError>;
}
