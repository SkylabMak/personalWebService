use std::sync::Arc;
use crate::application::services::profile::life_status::service::GetCurrentLifeStatusService;
use crate::infrastructure::repository_impl::profile::life_status::mysql_life_status_repo::MySqlLifeStatusRepository;

#[derive(Clone)]
pub struct LifeStatusUseCases {
    pub get_current: Arc<GetCurrentLifeStatusService<MySqlLifeStatusRepository>>,
}

impl LifeStatusUseCases {
    pub fn new(get_current: GetCurrentLifeStatusService<MySqlLifeStatusRepository>) -> Self {
        Self {
            get_current: Arc::new(get_current),
        }
    }
}
