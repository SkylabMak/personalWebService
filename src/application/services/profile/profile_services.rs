use crate::application::services::profile::life_status::service::GetCurrentLifeStatusService;
use crate::infrastructure::repositories::Repositories;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;

pub struct ProfileServices {
    pub life_status: GetCurrentLifeStatusService<LifeStatusRepositoryImpl>,
}

impl ProfileServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            life_status: GetCurrentLifeStatusService::new(repos.profile.life_status.clone()),
        }
    }
}
