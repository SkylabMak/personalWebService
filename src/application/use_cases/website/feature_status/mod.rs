pub mod dto;

use std::sync::Arc;
use crate::application::services::website::feature_status::FeatureStatusServices;
use crate::infrastructure::repository_impl::website::feature_status::repository::WebsiteRepositoryImpl;

#[derive(Clone)]
pub struct FeatureStatusUseCases {
    pub get_feature_status: Arc<crate::application::services::website::feature_status::service::GetWebsiteFeatureStatusService<WebsiteRepositoryImpl>>,
}

impl FeatureStatusUseCases {
    pub fn new(services: FeatureStatusServices) -> Self {
        Self {
            get_feature_status: Arc::new(services.get_feature_status),
        }
    }
}
