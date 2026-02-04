use async_trait::async_trait;
use google_cloud_storage::http::objects::delete::DeleteObjectRequest;
use google_cloud_storage::http::objects::download::Range;
use google_cloud_storage::http::objects::get::GetObjectRequest;
use google_cloud_storage::http::objects::upload::{Media, UploadObjectRequest, UploadType};
use crate::infrastructure::cloud_storage::gcs::common::gcs_repository::GcsRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::performance_content::performance_content_repository::PerformanceContentRepository;

#[derive(Clone)]
pub struct GcsPerformanceContentRepositoryImpl {
    gcs: GcsRepository,
}

impl GcsPerformanceContentRepositoryImpl {
    pub fn new(gcs: GcsRepository) -> Self {
        Self { gcs }
    }
}

#[async_trait]
impl PerformanceContentRepository for GcsPerformanceContentRepositoryImpl {
    async fn upload_content(
        &self,
        profile_id: &str,
        performance_id: &str,
        content: &str,
    ) -> Result<String, RepositoryError> {
        let path = format!("performance_content/{}/{}/content.md", profile_id, performance_id);
        
        let upload_type = UploadType::Simple(Media::new(path.clone()));
        let upload_request = UploadObjectRequest {
            bucket: self.gcs.bucket_name().to_string(),
            ..Default::default()
        };

        self.gcs.client().upload_object(&upload_request, content.as_bytes().to_vec(), &upload_type)
            .await
            .map_err(|e| RepositoryError::InternalError(format!("GCS Upload Error: {}", e)))?;

        Ok(format!("https://storage.googleapis.com/{}/{}", self.gcs.bucket_name(), path))
    }

    async fn get_content(
        &self,
        profile_id: &str,
        performance_id: &str,
    ) -> Result<String, RepositoryError> {
        let path = format!("performance_content/{}/{}/content.md", profile_id, performance_id);
        
        let data = self.gcs.client().download_object(&GetObjectRequest {
            bucket: self.gcs.bucket_name().to_string(),
            object: path,
            ..Default::default()
        }, &Range::default())
        .await
        .map_err(|e| RepositoryError::InternalError(format!("GCS Download Error: {}", e)))?;

        String::from_utf8(data)
            .map_err(|e| RepositoryError::InternalError(format!("UTF-8 Error: {}", e)))
    }

    async fn delete_content(
        &self,
        profile_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError> {
        let path = format!("performance_content/{}/{}/content.md", profile_id, performance_id);
        
        self.gcs.client().delete_object(&DeleteObjectRequest {
            bucket: self.gcs.bucket_name().to_string(),
            object: path,
            ..Default::default()
        })
        .await
        .map_err(|e| RepositoryError::InternalError(format!("GCS Delete Error: {}", e)))?;

        Ok(())
    }
}
