use crate::application::services::profile::life_status::service::GetCurrentLifeStatusService;
use crate::application::services::profile::announce::service::GetAnnounceListService;
use crate::infrastructure::repositories::Repositories;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;
use crate::infrastructure::repository_impl::profile::announce::repository::AnnounceRepositoryImpl;

pub struct ProfileServices {
    pub life_status: GetCurrentLifeStatusService<LifeStatusRepositoryImpl>,
    pub announce: GetAnnounceListService<AnnounceRepositoryImpl>,
}

impl ProfileServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            life_status: GetCurrentLifeStatusService::new(repos.profile.life_status.clone()),
            announce: GetAnnounceListService::new(repos.profile.announce.clone()),
        }
    }
}
