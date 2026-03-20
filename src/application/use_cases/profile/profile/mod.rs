pub mod dto;

use std::sync::Arc;
use crate::application::services::profile::service::GetProfileService;
use crate::infrastructure::repository_impl::profile::repository::ProfileRepositoryImpl;
use crate::infrastructure::repository_impl::profile::announce::repository::AnnounceRepositoryImpl;

use crate::infrastructure::repository_impl::profile::data::repository::ProfileDataRepositoryImpl;
use crate::infrastructure::repository_impl::profile::skill::repository::SkillRepositoryImpl;
use crate::infrastructure::repository_impl::profile::social::repository::SocialRepositoryImpl;

#[derive(Clone)]
pub struct ProfileBaseUseCases {
    pub get_one: Arc<GetProfileService<
        ProfileDataRepositoryImpl, 
        AnnounceRepositoryImpl,
        SkillRepositoryImpl,
        SocialRepositoryImpl
    >>,
}

impl ProfileBaseUseCases {
    pub fn new(get_one: GetProfileService<
        ProfileDataRepositoryImpl, 
        AnnounceRepositoryImpl,
        SkillRepositoryImpl,
        SocialRepositoryImpl
    >) -> Self {
        Self {
            get_one: Arc::new(get_one),
        }
    }
}
