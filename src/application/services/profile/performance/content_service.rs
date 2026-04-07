use async_trait::async_trait;
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::performance::dto::input::{
    UpdatePerformanceContentInput, GetPerformanceContentInput
};
use crate::interface_adapters::gateways::repositories::profile::performance::performance_repository::PerformanceRepository;
use crate::interface_adapters::gateways::repositories::profile::performance_content::performance_content_repository::PerformanceContentRepository;
use super::result::{PerformanceContentResult, PerformanceContentUpdateResult};
use crate::shared::utils::markdown::{parse_image_ids, strip_markdown};

pub struct GetPerformanceContentService<C>
where
    C: PerformanceContentRepository,
{
    content_repository: C,
}

impl<C> GetPerformanceContentService<C>
where
    C: PerformanceContentRepository,
{
    pub fn new(_repository: (), content_repository: C) -> Self {
        Self { content_repository }
    }
}

#[async_trait]
impl<C> UseCase for GetPerformanceContentService<C>
where
    C: PerformanceContentRepository + Send + Sync,
{
    type Input = GetPerformanceContentInput;
    type Output = PerformanceContentResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let content = self.content_repository
            .get_content(&input.profile_id, &input.performance_id)
            .await
            .map_app_err("Failed to fetch performance content")?;

        Ok(PerformanceContentResult {
            content_markdown: content,
        })
    }
}

pub struct UpdatePerformanceContentService<R, C>
where
    R: PerformanceRepository,
    C: PerformanceContentRepository,
{
    repository: R,
    content_repository: C,
}

impl<R, C> UpdatePerformanceContentService<R, C>
where
    R: PerformanceRepository,
    C: PerformanceContentRepository,
{
    pub fn new(repository: R, content_repository: C) -> Self {
        Self { repository, content_repository }
    }
}

#[async_trait]
impl<R, C> UseCase for UpdatePerformanceContentService<R, C>
where
    R: PerformanceRepository + Send + Sync,
    C: PerformanceContentRepository + Send + Sync,
{
    type Input = UpdatePerformanceContentInput;
    type Output = PerformanceContentUpdateResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let old_perf = self.repository
            .find_by_id(&input.performance_id)
            .await
            .map_app_err("Failed to fetch existing performance")?
            .ok_or_else(|| ApplicationError::NotFound { resource: "Performance", identifier: input.performance_id.clone() })?;

        let content_url = self.content_repository
            .update_content(&input.profile_id, &input.performance_id, &input.content_markdown)
            .await
            .map_app_err("Failed to update performance content")?;

        let stripped_content = strip_markdown(&input.content_markdown);
        let content_preview = if stripped_content.len() > 500 {
            Some(stripped_content[..500].to_string())
        } else {
            Some(stripped_content)
        };

        // Update performance with new preview and content_url
        let mut updated_perf = old_perf.clone();
        updated_perf.content_url = Some(content_url.clone());
        updated_perf.content_preview = content_preview;
        updated_perf.updated_at = Some(sqlx::types::chrono::Utc::now().format("%Y-%m-%d").to_string());

        self.repository
            .update(updated_perf)
            .await
            .map_app_err("Failed to update performance metadata")?;

        // Image tracking
        let image_ids = parse_image_ids(&input.content_markdown);
        self.repository
            .sync_image_usage(&input.performance_id, &image_ids)
            .await
            .map_app_err("Failed to sync image usage")?;

        Ok(PerformanceContentUpdateResult {
            performance_id: input.performance_id,
            content_url,
            images_synced: image_ids.len(),
        })
    }
}
