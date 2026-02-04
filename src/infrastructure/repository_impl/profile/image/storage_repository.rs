use async_trait::async_trait;
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use crate::infrastructure::cloud_storage::gcs::common::gcs_repository::GcsRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::image::image_storage_repository::ImageStorageRepository;

#[derive(Clone)]
pub struct GcsImageStorageRepositoryImpl {
    gcs: GcsRepository,
}

impl GcsImageStorageRepositoryImpl {
    pub fn new(gcs: GcsRepository) -> Self {
        Self { gcs }
    }
}

#[async_trait]
impl ImageStorageRepository for GcsImageStorageRepositoryImpl {
    async fn upload_image(
        &self,
        profile_id: &str,
        filename: &str,
        data: Vec<u8>,
        _content_type: &str,
    ) -> Result<String, RepositoryError> {
        let path = format!("performance_image/{}/{}", profile_id, filename);
        
        let upload_type = UploadType::Simple(Media::new(path.clone()));
        let upload_request = UploadObjectRequest {
            bucket: self.gcs.bucket_name().to_string(),
            ..Default::default()
        };

        self.gcs.client().upload_object(&upload_request, data, &upload_type)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("GCS Upload Error: {}", e)))?;

        Ok(format!("https://storage.googleapis.com/{}/{}", self.gcs.bucket_name(), path))
    }

    async fn delete_image(
        &self,
        profile_id: &str,
        filename: &str,
    ) -> Result<(), RepositoryError> {
        let path = format!("performance_image/{}/{}", profile_id, filename);
        
        self.gcs.client().delete_object(&DeleteObjectRequest {
            bucket: self.gcs.bucket_name().to_string(),
            object: path,
            ..Default::default()
        })
        .await
        .map_err(|e| RepositoryError::InternalError(format!("GCS Delete Error: {}", e)))?;

        Ok(())
    }

    async fn get_image_url(
        &self,
        profile_id: &str,
        filename: &str,
    ) -> Result<String, RepositoryError> {
        let path = format!("performance_image/{}/{}", profile_id, filename);
        Ok(format!("https://storage.googleapis.com/{}/{}", self.gcs.bucket_name(), path))
    }
}
