use async_trait::async_trait;

use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::life_status::dto::input::GetLifeStatusInput;
use crate::interface_adapters::gateways::repositories::profile::life_status::life_status_repository::LifeStatusRepository;
use super::result::LifeStatusResult;

pub struct GetCurrentLifeStatusService<R>
where
    R: LifeStatusRepository,
{
    repository: R,
}

impl<R> GetCurrentLifeStatusService<R>
where
    R: LifeStatusRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetCurrentLifeStatusService<R>
where
    R: LifeStatusRepository + Send + Sync,
{
    type Input = GetLifeStatusInput;
    type Output = LifeStatusResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let data = self
            .repository
            .find_current_by_profile_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch life status")?
            .ok_or_else(|| ApplicationError::NotFound {
                resource: "LifeStatus",
                identifier: input.profile_id,
            })?;

        Ok(LifeStatusResult {
            name: data.name,
            description: data.description,
            color_token: data.color_token,
        })
    }
}
