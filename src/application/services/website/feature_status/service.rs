use async_trait::async_trait;

use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::application::feature_status::dto::input::GetAppFeatureStatusInput;
use crate::interface_adapters::gateways::repositories::application::feature_status::app_repository::AppRepository;
use crate::application::services::website::feature_status::result::{FeatureStatusResult, WebsiteFeatureStatusesResult};

pub struct GetWebsiteFeatureStatusService<R>
where
    R: AppRepository,
{
    repository: R,
}

impl<R> GetWebsiteFeatureStatusService<R>
where
    R: AppRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetWebsiteFeatureStatusService<R>
where
    R: AppRepository + Send + Sync,
{
    type Input = GetAppFeatureStatusInput;
    type Output = WebsiteFeatureStatusesResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let data = self
            .repository
            .find_feature_statuses_by_website_id(&input.website_id)
            .await
            .map_app_err("Failed to fetch application feature status")?;

        let features = data
            .into_iter()
            .map(|f| FeatureStatusResult {
                feature_code: f.feature_code,
                feature_name: f.feature_name,
                status_code: f.status_code,
                status_name: f.status_name,
                is_closed: f.is_closed,
                updated_at: f.updated_at,
                note: f.note,
            })
            .collect();

        Ok(WebsiteFeatureStatusesResult { features })
    }
}
