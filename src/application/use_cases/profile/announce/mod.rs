pub mod dto;

use std::sync::Arc;
use crate::application::services::profile::announce::service::GetAnnounceListService;
use crate::infrastructure::repository_impl::profile::announce::repository::AnnounceRepositoryImpl;

#[derive(Clone)]
pub struct AnnounceUseCases {
    pub get_list: Arc<GetAnnounceListService<AnnounceRepositoryImpl>>,
}

impl AnnounceUseCases {
    pub fn new(get_list: GetAnnounceListService<AnnounceRepositoryImpl>) -> Self {
        Self {
            get_list: Arc::new(get_list),
        }
    }
}
