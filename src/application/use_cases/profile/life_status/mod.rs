pub mod dto;

use std::sync::Arc;
use crate::application::services::profile::life_status::service::GetCurrentLifeStatusService;
use crate::infrastructure::repository_impl::profile::life_status::repository::LifeStatusRepositoryImpl;

#[derive(Clone)]
pub struct LifeStatusUseCases {
    pub get_current: Arc<GetCurrentLifeStatusService<LifeStatusRepositoryImpl>>,
}

impl LifeStatusUseCases {
    pub fn new(get_current: GetCurrentLifeStatusService<LifeStatusRepositoryImpl>) -> Self {
        Self {
            get_current: Arc::new(get_current),
        }
    }
}
