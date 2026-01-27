use async_trait::async_trait;

use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::announce::dto::input::GetAnnounceListInput;
use crate::interface_adapters::gateways::repositories::profile::announce::announce_repository::AnnounceRepository;
use super::result::AnnounceResult;

pub struct GetAnnounceListService<R>
where
    R: AnnounceRepository,
{
    repository: R,
}

impl<R> GetAnnounceListService<R>
where
    R: AnnounceRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetAnnounceListService<R>
where
    R: AnnounceRepository + Send + Sync,
{
    type Input = GetAnnounceListInput;
    type Output = AnnounceResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        input.validate().map_err(|e| ApplicationError::ValidationError { message: e })?;

        let data = self
            .repository
            .find_active_by_profile_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch announce list")?;

        Ok(AnnounceResult {
            items: data,
        })
    }
}
