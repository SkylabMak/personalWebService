use async_trait::async_trait;
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::profile::dto::input::GetProfileInput;
use crate::interface_adapters::gateways::repositories::profile::profile_repository::ProfileRepository;
use super::result::ProfileResult;

pub struct GetProfileService<R>
where
    R: ProfileRepository,
{
    repository: R,
}

impl<R> GetProfileService<R>
where
    R: ProfileRepository,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl<R> UseCase for GetProfileService<R>
where
    R: ProfileRepository + Send + Sync,
{
    type Input = GetProfileInput;
    type Output = ProfileResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let profile = self.repository
            .find_by_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch profile")?
            .ok_or_else(|| ApplicationError::NotFound { 
                resource: "Profile", 
                identifier: input.profile_id 
            })?;

        Ok(ProfileResult { profile })
    }
}
