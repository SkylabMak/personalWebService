use crate::application::services::profile::life_status::service::GetCurrentLifeStatusService;
use crate::infrastructure::repositories::Repositories;
use crate::infrastructure::repository_impl::profile::life_status::mysql_life_status_repo::MySqlLifeStatusRepository;

pub struct ProfileServices {
    pub life_status: GetCurrentLifeStatusService<MySqlLifeStatusRepository>,
}

impl ProfileServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            life_status: GetCurrentLifeStatusService::new(repos.profile.life_status.clone()),
        }
    }
}
