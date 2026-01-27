pub mod feature_status;

use crate::application::services::website::website_services::WebsiteServices;
use crate::application::use_cases::application::feature_status::FeatureStatusUseCases;

#[derive(Clone)]
pub struct WebsiteUseCases {
    pub feature_status: FeatureStatusUseCases,
}

impl WebsiteUseCases {
    pub fn new(services: WebsiteServices) -> Self {
        Self {
            feature_status: FeatureStatusUseCases::new(services.feature_status),
        }
    }
}
