use async_trait::async_trait;
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::use_cases::use_case::UseCase;
use crate::application::use_cases::profile::profile::dto::input::GetProfileInput;
use crate::interface_adapters::gateways::repositories::profile::profile_repository::ProfileRepository;
use crate::interface_adapters::gateways::repositories::profile::announce::announce_repository::AnnounceRepository;
use crate::interface_adapters::gateways::repositories::profile::skill::skill_repository::SkillRepository;
use crate::interface_adapters::gateways::repositories::profile::social::social_repository::SocialRepository;
use super::result::ProfileResult;

pub struct GetProfileService<PR, AR, SKR, SOR>
where
    PR: ProfileRepository,
    AR: AnnounceRepository,
    SKR: SkillRepository,
    SOR: SocialRepository,
{
    profile_repository: PR,
    announce_repository: AR,
    skill_repository: SKR,
    social_repository: SOR,
}

impl<PR, AR, SKR, SOR> GetProfileService<PR, AR, SKR, SOR>
where
    PR: ProfileRepository,
    AR: AnnounceRepository,
    SKR: SkillRepository,
    SOR: SocialRepository,
{
    pub fn new(
        profile_repository: PR, 
        announce_repository: AR,
        skill_repository: SKR,
        social_repository: SOR,
    ) -> Self {
        Self { 
            profile_repository,
            announce_repository,
            skill_repository,
            social_repository,
        }
    }
}

#[async_trait]
impl<PR, AR, SKR, SOR> UseCase for GetProfileService<PR, AR, SKR, SOR>
where
    PR: ProfileRepository + Send + Sync,
    AR: AnnounceRepository + Send + Sync,
    SKR: SkillRepository + Send + Sync,
    SOR: SocialRepository + Send + Sync,
{
    type Input = GetProfileInput;
    type Output = ProfileResult;
    type Error = ApplicationError;

    async fn execute(&self, input: Self::Input) -> Result<Self::Output, Self::Error> {
        let profile_id = input.profile_id.clone();
        let profile = self.profile_repository
            .find_by_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch profile")?
            .ok_or_else(|| {
                ApplicationError::NotFound { 
                    resource: "Profile", 
                    identifier: profile_id 
                }
            })?;

        let announces = self.announce_repository
            .find_active_by_profile_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch announces")?;

        let skills = self.skill_repository
            .find_by_profile_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch skills")?;

        let socials = self.social_repository
            .find_by_profile_id(&input.profile_id)
            .await
            .map_app_err("Failed to fetch socials")?;

        Ok(ProfileResult { 
            profile,
            announces,
            skills,
            socials,
        })
    }
}
