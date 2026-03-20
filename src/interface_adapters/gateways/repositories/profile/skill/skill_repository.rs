use async_trait::async_trait;
use crate::domain::entities::profile::skill::skill::Skill;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;

#[async_trait]
pub trait SkillRepository: Send + Sync {
    async fn find_by_profile_id(&self, profile_id: &str) -> Result<Vec<Skill>, RepositoryError>;
}
