use async_trait::async_trait;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait ImageStorageRepository: Send + Sync {
    async fn upload_image(
        &self,
        profile_id: &str,
        filename: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<String, RepositoryError>;

    async fn delete_image(
        &self,
        profile_id: &str,
        filename: &str,
    ) -> Result<(), RepositoryError>;

    async fn get_image_url(
        &self,
        profile_id: &str,
        filename: &str,
    ) -> Result<String, RepositoryError>;
}
