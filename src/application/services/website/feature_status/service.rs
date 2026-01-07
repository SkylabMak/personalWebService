use async_trait::async_trait;

use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::website::feature_status::dto::input::GetWebsiteFeatureStatusInput;
use crate::interface_adapters::gateways::repositories::website::feature_status::website_repository::WebsiteRepository;
use crate::application::services::website::feature_status::result::{FeatureStatusResult, WebsiteFeatureStatusesResult};

pub struct GetWebsiteFeatureStatusService<R>
where
    R: WebsiteRepository,
{
    repository: R,
}

impl<R> GetWebsiteFeatureStatusService<R>
where
    R: WebsiteRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetWebsiteFeatureStatusService<R>
where
    R: WebsiteRepository + Send + Sync,
{
    type Input = GetWebsiteFeatureStatusInput;
    type Output = WebsiteFeatureStatusesResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let data = self
            .repository
            .find_feature_statuses_by_website_id(&input.website_id)
            .await
            .map_app_err("Failed to fetch website feature status")?;

        let features = data
            .into_iter()
            .map(|f| FeatureStatusResult {
                feature_code: f.feature_code,
                feature_name: f.feature_name,
                status_name: f.status_name,
                is_closed: f.is_closed,
                updated_at: f.updated_at,
                note: f.note,
            })
            .collect();

        Ok(WebsiteFeatureStatusesResult { features })
    }
}
