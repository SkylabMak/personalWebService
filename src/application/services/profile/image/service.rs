use async_trait::async_trait;
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::image::dto::input::{
    GetImagesInput, GetImageInput, CreateImageInput, UpdateImageMetadataInput,
    DeleteImageInput, ForceDeleteImageInput, GetUnusedImagesInput, DeleteUnusedImagesInput, TrackImageUsageInput
};
use crate::interface_adapters::gateways::repositories::profile::image::image_repository::ImageRepository;
use crate::interface_adapters::gateways::repositories::profile::image::image_storage_repository::ImageStorageRepository;
use crate::application::services::profile::image::result::{
    ImageListResult, ImageResult, ImageUsageResult, PerformanceUsageInfo,
    UnusedImagesResult, DeleteUnusedImagesResult, MessageResult
};
use crate::domain::entities::profile::image::image::Image;

pub struct GetImagesService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> GetImagesService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetImagesService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = GetImagesInput;
    type Output = ImageListResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let limit = input.limit.unwrap_or(20);
        let offset = input.offset.unwrap_or(0);

        let (images_data, total) = self
            .repository
            .find_all_by_profile_id(&input.profile_id, input.search, limit, offset)
            .await
            .map_app_err("Failed to fetch images")?;

        let images = images_data
            .into_iter()
            .map(|(img, usage)| ImageResult {
                id: img.id,
                storage_url: img.storage_url,
                filename: img.filename,
                original_filename: img.original_filename,
                width: img.width,
                height: img.height,
                file_size: img.file_size,
                mime_type: img.mime_type,
                alt_text: img.alt_text,
                caption: img.caption,
                created_at: img.created_at,
                usage_count: Some(usage),
            })
            .collect();

        Ok(ImageListResult {
            images,
            total,
            limit,
            offset,
        })
    }
}

pub struct GetImageService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> GetImageService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetImageService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = GetImageInput;
    type Output = ImageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let (img, usage) = self
            .repository
            .find_by_id_and_profile_id(&input.id, &input.profile_id)
            .await
            .map_app_err("Failed to fetch image")?
            .ok_or_else(|| ApplicationError::NotFound {
                resource: "Image",
                identifier: input.id,
            })?;

        Ok(ImageResult {
            id: img.id,
            storage_url: img.storage_url,
            filename: img.filename,
            original_filename: img.original_filename,
            width: img.width,
            height: img.height,
            file_size: img.file_size,
            mime_type: img.mime_type,
            alt_text: img.alt_text,
            caption: img.caption,
            created_at: img.created_at,
            usage_count: Some(usage),
        })
    }
}

pub struct GetImageUsageService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> GetImageUsageService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetImageUsageService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = GetImageInput; // Reuse GetImageInput for single image usage
    type Output = ImageUsageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let usages = self
            .repository
            .find_usage_by_image_id(&input.id, &input.profile_id)
            .await
            .map_app_err("Failed to fetch image usage")?;

        let mut total_usage = 0;
        let performances = usages
            .into_iter()
            .map(|u| {
                total_usage += u.usage_count;
                PerformanceUsageInfo {
                    performance_id: u.performance_id,
                    title: u.title,
                    usage_count: u.usage_count,
                    first_used_at: u.first_used_at,
                    last_used_at: u.last_used_at,
                }
            })
            .collect();

        Ok(ImageUsageResult {
            image_id: input.id,
            total_usage,
            performances,
        })
    }
}

pub struct CreateImageService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    repository: R,
    storage_repository: S,
}

impl<R, S> CreateImageService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    pub fn new(repository: R, storage_repository: S) -> Self {
        Self { repository, storage_repository }
    }
}

#[async_trait]
impl<R, S> UseCase for CreateImageService<R, S>
where
    R: ImageRepository + Send + Sync,
    S: ImageStorageRepository + Send + Sync,
{
    type Input = CreateImageInput;
    type Output = ImageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        // Extract metadata from image bytes
        let (width, height, mime_type) = match image::ImageReader::new(std::io::Cursor::new(&input.image_bytes))
            .with_guessed_format()
            .map_err(|e| ApplicationError::ValidationError { message: format!("Failed to read image: {}", e) })?
            .decode() {
                Ok(img) => {
                    use image::GenericImageView;
                    let (w, h) = img.dimensions();
                    // Try to get mime type from format
                    let format = image::guess_format(&input.image_bytes)
                        .unwrap_or(image::ImageFormat::Jpeg);
                    let mime = format.to_mime_type().to_string();
                    (Some(w as i32), Some(h as i32), mime)
                },
                Err(_) => {
                    // Fallback if decoding fails but we still want to try saving
                    (None, None, input.mime_type.clone())
                }
            };

        let id = uuid::Uuid::new_v4().to_string();
        let extension = input.original_filename.split('.').last().unwrap_or("jpg");
        let filename = format!("{}.{}", id, extension);
        
        // 1. Upload to storage
        let storage_url = self.storage_repository
            .upload_image(&input.profile_id, &filename, input.image_bytes, &mime_type)
            .await
            .map_app_err("Failed to upload image to storage")?;

        let created_at = sqlx::types::chrono::Utc::now().to_rfc3339();

        let image = Image {
            id: id.clone(),
            profile_id: input.profile_id,
            filename: filename.clone(),
            original_filename: input.original_filename.clone(),
            storage_url: storage_url.clone(),
            file_size: input.file_size,
            width,
            height,
            mime_type: mime_type.clone(),
            alt_text: input.alt_text.clone(),
            caption: input.caption.clone(),
            created_at: created_at.clone(),
        };

        // 2. Save metadata to database
        self.repository
            .create(image)
            .await
            .map_app_err("Failed to create image in database")?;

        Ok(ImageResult {
            id: id.clone(),
            storage_url,
            filename,
            original_filename: input.original_filename,
            width,
            height,
            file_size: input.file_size,
            mime_type,
            alt_text: input.alt_text,
            caption: input.caption,
            created_at,
            usage_count: Some(0),
        })
    }
}

pub struct TrackImageUsageService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> TrackImageUsageService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for TrackImageUsageService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = TrackImageUsageInput;
    type Output = MessageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        self.repository
            .track_usage(&input.image_id, &input.performance_id)
            .await
            .map_app_err("Failed to track image usage")?;

        Ok(MessageResult {
            message: "Image usage tracked successfully".to_string(),
            id: Some(input.image_id),
        })
    }
}

pub struct UntrackImageUsageService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> UntrackImageUsageService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for UntrackImageUsageService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = TrackImageUsageInput;
    type Output = MessageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        self.repository
            .untrack_usage(&input.image_id, &input.performance_id)
            .await
            .map_app_err("Failed to untrack image usage")?;

        Ok(MessageResult {
            message: "Image usage untracked successfully".to_string(),
            id: Some(input.image_id),
        })
    }
}

pub struct UpdateImageMetadataService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> UpdateImageMetadataService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for UpdateImageMetadataService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = UpdateImageMetadataInput;
    type Output = MessageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        self.repository
            .update_metadata(&input.id, &input.profile_id, input.alt_text, input.caption)
            .await
            .map_app_err("Failed to update image metadata")?;

        Ok(MessageResult {
            message: "Image metadata updated successfully".to_string(),
            id: Some(input.id),
        })
    }
}

pub struct DeleteImageService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    repository: R,
    storage_repository: S,
}

impl<R, S> DeleteImageService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    pub fn new(repository: R, storage_repository: S) -> Self {
        Self { repository, storage_repository }
    }
}

#[async_trait]
impl<R, S> UseCase for DeleteImageService<R, S>
where
    R: ImageRepository + Send + Sync,
    S: ImageStorageRepository + Send + Sync,
{
    type Input = DeleteImageInput;
    type Output = MessageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        // 1. Check if image exists and get metadata for GCS deletion
        let (image, usage) = self.repository
            .find_by_id_and_profile_id(&input.id, &input.profile_id)
            .await
            .map_app_err("Failed to check image existence")?
            .ok_or_else(|| ApplicationError::NotFound {
                resource: "Image",
                identifier: input.id.clone(),
            })?;

        if usage > 0 {
            return Err(ApplicationError::ValidationError { 
                message: format!("Image is currently used in {} performances. Delete performances first.", usage) 
            });
        }

        // 2. Delete from storage
        self.storage_repository
            .delete_image(&input.profile_id, &image.filename)
            .await
            .map_app_err("Failed to delete image from storage")?;

        // 3. Delete from database
        self.repository
            .delete(&input.id, &input.profile_id)
            .await
            .map_app_err("Failed to delete image from database")?;

        Ok(MessageResult {
            message: "Image deleted successfully".to_string(),
            id: Some(input.id),
        })
    }
}

pub struct ForceDeleteImageService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    repository: R,
    storage_repository: S,
}

impl<R, S> ForceDeleteImageService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    pub fn new(repository: R, storage_repository: S) -> Self {
        Self { repository, storage_repository }
    }
}

#[async_trait]
impl<R, S> UseCase for ForceDeleteImageService<R, S>
where
    R: ImageRepository + Send + Sync,
    S: ImageStorageRepository + Send + Sync,
{
    type Input = ForceDeleteImageInput;
    type Output = MessageResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        // 1. Check if image exists and get metadata for GCS deletion
        let (image, _) = self.repository
            .find_by_id_and_profile_id(&input.id, &input.profile_id)
            .await
            .map_app_err("Failed to check image existence")?
            .ok_or_else(|| ApplicationError::NotFound {
                resource: "Image",
                identifier: input.id.clone(),
            })?;

        // 2. Delete from storage
        self.storage_repository
            .delete_image(&input.profile_id, &image.filename)
            .await
            .map_app_err("Failed to delete image from storage")?;

        // 3. Force delete from database (clears usage and image record)
        self.repository
            .force_delete(&input.id, &input.profile_id)
            .await
            .map_app_err("Failed to force delete image from database")?;

        Ok(MessageResult {
            message: "Image and its usage records deleted successfully".to_string(),
            id: Some(input.id),
        })
    }
}

pub struct GetUnusedImagesService<R>
where
    R: ImageRepository,
{
    repository: R,
}

impl<R> GetUnusedImagesService<R>
where
    R: ImageRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetUnusedImagesService<R>
where
    R: ImageRepository + Send + Sync,
{
    type Input = GetUnusedImagesInput;
    type Output = UnusedImagesResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let images = self.repository
            .find_unused_by_profile_id(&input.profile_id, input.days_old)
            .await
            .map_app_err("Failed to fetch unused images")?;

        let total_size_bytes = images.iter().map(|img| img.file_size as i64).sum();
        let count = images.len();

        let unused_images = images.into_iter().map(|img| ImageResult {
            id: img.id,
            storage_url: img.storage_url,
            filename: img.filename,
            original_filename: img.original_filename,
            width: img.width,
            height: img.height,
            file_size: img.file_size,
            mime_type: img.mime_type,
            alt_text: img.alt_text,
            caption: img.caption,
            created_at: img.created_at,
            usage_count: Some(0),
        }).collect();

        Ok(UnusedImagesResult {
            unused_images,
            total_size_bytes,
            count,
        })
    }
}

pub struct DeleteUnusedImagesService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    repository: R,
    storage_repository: S,
}

impl<R, S> DeleteUnusedImagesService<R, S>
where
    R: ImageRepository,
    S: ImageStorageRepository,
{
    pub fn new(repository: R, storage_repository: S) -> Self {
        Self { repository, storage_repository }
    }
}

#[async_trait]
impl<R, S> UseCase for DeleteUnusedImagesService<R, S>
where
    R: ImageRepository + Send + Sync,
    S: ImageStorageRepository + Send + Sync,
{
    type Input = DeleteUnusedImagesInput;
    type Output = DeleteUnusedImagesResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        // 1. Get list of unused images first to delete from storage
        let unused_images = self.repository
            .find_unused_by_profile_id(&input.profile_id, input.days_old)
            .await
            .map_app_err("Failed to fetch unused images for deletion")?;

        // 2. Delete from storage
        for image in &unused_images {
            let _ = self.storage_repository
                .delete_image(&input.profile_id, &image.filename)
                .await;
            // We ignore storage errors during bulk delete to ensure we try to delete all
            // or we could collect errors, but usually for "unused" cleanup, we want to proceed.
        }

        // 3. Delete from database
        let (deleted_count, freed_bytes) = self.repository
            .delete_unused_by_profile_id(&input.profile_id, input.days_old)
            .await
            .map_app_err("Failed to delete unused images from database")?;

        Ok(DeleteUnusedImagesResult {
            deleted_count,
            freed_bytes,
        })
    }
}
