pub mod dto;

use std::sync::Arc;
use crate::application::services::website::feature_status::FeatureStatusServices;
use crate::infrastructure::repository_impl::application::feature_status::repository::AppRepositoryImpl;

#[derive(Clone)]
pub struct FeatureStatusUseCases {
    pub get_feature_status: Arc<crate::application::services::website::feature_status::service::GetWebsiteFeatureStatusService<AppRepositoryImpl>>,
}

impl FeatureStatusUseCases {
    pub fn new(services: FeatureStatusServices) -> Self {
        Self {
            get_feature_status: Arc::new(services.get_feature_status),
        }
    }
}
