use async_trait::async_trait;
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::performance::dto::input::{
    CreatePerformanceInput, UpdatePerformanceInput, DeletePerformanceInput,
    ListPerformancesInput
};
use crate::interface_adapters::gateways::repositories::profile::performance::performance_repository::PerformanceRepository;
use crate::interface_adapters::gateways::repositories::profile::performance_content::performance_content_repository::PerformanceContentRepository;
use super::result::{
    PerformanceResult, PerformanceUpdateResult, PerformanceDeleteResult,
    PerformanceListResult
};

pub struct ListPerformancesService<R>
where
    R: PerformanceRepository,
{
    repository: R,
}

impl<R> ListPerformancesService<R>
where
    R: PerformanceRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for ListPerformancesService<R>
where
    R: PerformanceRepository + Send + Sync,
{
    type Input = ListPerformancesInput;
    type Output = PerformanceListResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let performances = self.repository
            .find_by_profile_id(&input.profile_id, input.visibility_id.as_deref())
            .await
            .map_app_err("Failed to fetch performances")?;

        Ok(PerformanceListResult { performances })
    }
}
use crate::domain::entities::profile::performance::performance::Performance;

pub struct CreatePerformanceService<R, C>
where
    R: PerformanceRepository,
    C: PerformanceContentRepository,
{
    repository: R,
    content_repository: C,
}

impl<R, C> CreatePerformanceService<R, C>
where
    R: PerformanceRepository,
    C: PerformanceContentRepository,
{
    pub fn new(repository: R, content_repository: C) -> Self {
        Self { repository, content_repository }
    }
}

#[async_trait]
impl<R, C> UseCase for CreatePerformanceService<R, C>
where
    R: PerformanceRepository + Send + Sync,
    C: PerformanceContentRepository + Send + Sync,
{
    type Input = CreatePerformanceInput;
    type Output = PerformanceResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let id = uuid::Uuid::new_v4().to_string();
        let created_at = sqlx::types::chrono::Utc::now().format("%Y-%m-%d").to_string();

        // Create empty markdown in GCS
        let content_url = self.content_repository
            .upload_content(&input.profile_id, &id, "")
            .await
            .map_app_err("Failed to create initial performance content")?;

        let perf = Performance {
            id: id.clone(),
            profile_id: input.profile_id,
            category_id: input.category_id,
            visibility_id: input.visibility_id,
            title: input.title.clone(),
            summary: input.summary,
            content_url: Some(content_url),
            content_type: "markdown".to_string(),
            content_preview: Some("".to_string()),
            start_date: input.start_date,
            end_date: input.end_date,
            location: input.location,
            close: false,
            created_at: created_at.clone(),
            updated_at: None,
        };

        let created_perf = self.repository
            .create(perf)
            .await
            .map_app_err("Failed to create performance")?;

        Ok(PerformanceResult {
            id: created_perf.id.clone(),
            title: created_perf.title,
            content_url: created_perf.content_url,
            images_tracked: 0,
            created_at: created_perf.created_at,
        })
    }
}

pub struct UpdatePerformanceService<R>
where
    R: PerformanceRepository,
{
    repository: R,
}

impl<R> UpdatePerformanceService<R>
where
    R: PerformanceRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for UpdatePerformanceService<R>
where
    R: PerformanceRepository + Send + Sync,
{
    type Input = UpdatePerformanceInput;
    type Output = PerformanceUpdateResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let old_perf = self.repository
            .find_by_id(&input.id)
            .await
            .map_app_err("Failed to fetch existing performance")?
            .ok_or_else(|| ApplicationError::NotFound { resource: "Performance", identifier: input.id.clone() })?;

        let updated_at = sqlx::types::chrono::Utc::now().format("%Y-%m-%d").to_string();

        let perf = Performance {
            id: input.id.clone(),
            profile_id: input.profile_id,
            category_id: input.category_id,
            visibility_id: input.visibility_id,
            title: input.title.clone(),
            summary: input.summary,
            content_url: old_perf.content_url,
            content_type: old_perf.content_type,
            content_preview: old_perf.content_preview,
            start_date: input.start_date,
            end_date: input.end_date,
            location: input.location,
            close: input.close,
            created_at: old_perf.created_at,
            updated_at: Some(updated_at.clone()),
        };

        self.repository
            .update(perf)
            .await
            .map_app_err("Failed to update performance")?;

        Ok(PerformanceUpdateResult {
            id: input.id,
            title: input.title,
            images_added: 0,
            images_removed: 0,
            updated_at,
        })
    }
}

pub struct DeletePerformanceService<R, C>
where
    R: PerformanceRepository,
    C: PerformanceContentRepository,
{
    repository: R,
    content_repository: C,
}

impl<R, C> DeletePerformanceService<R, C>
where
    R: PerformanceRepository,
    C: PerformanceContentRepository,
{
    pub fn new(repository: R, content_repository: C) -> Self {
        Self { repository, content_repository }
    }
}

#[async_trait]
impl<R, C> UseCase for DeletePerformanceService<R, C>
where
    R: PerformanceRepository + Send + Sync,
    C: PerformanceContentRepository + Send + Sync,
{
    type Input = DeletePerformanceInput;
    type Output = PerformanceDeleteResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        // Delete content in GCS
        self.content_repository
            .delete_content(&input.profile_id, &input.id)
            .await
            .map_app_err("Failed to delete performance content from storage")?;

        // Delete image usage associations first
        self.repository
            .delete_image_usage_by_performance_id(&input.id)
            .await
            .map_app_err("Failed to delete performance image usage")?;

        self.repository
            .delete(&input.id)
            .await
            .map_app_err("Failed to delete performance")?;

        Ok(PerformanceDeleteResult {
            message: "Performance deleted".to_string(),
            deleted_id: input.id,
        })
    }
}
