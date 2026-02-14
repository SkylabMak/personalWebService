pub mod dto;

use std::sync::Arc;
use crate::application::services::profile::service::GetProfileService;
use crate::infrastructure::repository_impl::profile::repository::ProfileRepositoryImpl;

#[derive(Clone)]
pub struct ProfileBaseUseCases {
    pub get_one: Arc<GetProfileService<ProfileRepositoryImpl>>,
}

impl ProfileBaseUseCases {
    pub fn new(get_one: GetProfileService<ProfileRepositoryImpl>) -> Self {
        Self {
            get_one: Arc::new(get_one),
        }
    }
}
