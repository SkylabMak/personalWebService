use async_trait::async_trait;

use crate::application::errors::ApplicationError;
use crate::application::use_cases::use_case::UseCase;
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
    type Input = String;
    type Output = LifeStatusResult;
    type Error = ApplicationError;

    async fn execute(&self, profile_id: Self::Input) -> Result<Self::Output, Self::Error> {
        let data = self
            .repository
            .find_current_by_profile_id(&profile_id)
            .await
            .map_err(|_| ApplicationError::Unexpected {
                message: "Failed to fetch life status".to_string(),
            })?
            .ok_or_else(|| ApplicationError::NotFound {
                resource: "LifeStatus",
                identifier: profile_id,
            })?;

        Ok(LifeStatusResult {
            name: data.name,
            description: data.description,
            color_token: data.color_token,
        })
    }
}
