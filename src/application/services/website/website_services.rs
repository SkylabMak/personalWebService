use crate::infrastructure::repositories::Repositories;
use crate::application::services::website::feature_status::FeatureStatusServices;

pub struct WebsiteServices {
    pub feature_status: FeatureStatusServices,
}

impl WebsiteServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            feature_status: FeatureStatusServices::new(repos),
        }
    }
}
