use async_trait::async_trait;
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::performance::dto::input::{
    UpdatePerformanceContentInput, GetPerformanceContentInput
};
use crate::interface_adapters::gateways::repositories::profile::performance::performance_repository::PerformanceRepository;
use crate::interface_adapters::gateways::repositories::profile::performance_content::performance_content_repository::PerformanceContentRepository;
use super::result::{PerformanceContentResult, PerformanceContentUpdateResult};
use crate::shared::utils::markdown::parse_image_ids;
use std::collections::HashSet;

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
            .upload_content(&input.profile_id, &input.performance_id, &input.content_markdown)
            .await
            .map_app_err("Failed to update performance content")?;

        let content_preview = if input.content_markdown.len() > 500 {
            Some(input.content_markdown[..500].to_string())
        } else {
            Some(input.content_markdown.clone())
        };

        // Update performance with new preview and content_url
        let mut updated_perf = old_perf.clone();
        updated_perf.content_url = Some(content_url.clone());
        updated_perf.content_preview = content_preview;
        updated_perf.updated_at = Some(sqlx::types::chrono::Utc::now().to_rfc3339());

        self.repository
            .update(updated_perf)
            .await
            .map_app_err("Failed to update performance metadata")?;

        // Image tracking
        let old_image_ids: HashSet<String> = parse_image_ids(old_perf.content_preview.as_deref().unwrap_or("")).into_iter().collect();
        let new_image_ids: HashSet<String> = parse_image_ids(&input.content_markdown).into_iter().collect();

        let mut images_added = 0;
        let mut images_removed = 0;

        for img_id in new_image_ids.difference(&old_image_ids) {
            self.repository
                .track_image_usage(img_id, &input.performance_id)
                .await
                .map_app_err("Failed to track new image usage")?;
            images_added += 1;
        }

        for img_id in old_image_ids.difference(&new_image_ids) {
            self.repository
                .untrack_image_usage(img_id, &input.performance_id)
                .await
                .map_app_err("Failed to untrack removed image usage")?;
            images_removed += 1;
        }

        Ok(PerformanceContentUpdateResult {
            performance_id: input.performance_id,
            content_url,
            images_added,
            images_removed,
        })
    }
}
