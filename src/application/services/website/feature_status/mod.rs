use service::GetWebsiteFeatureStatusService;
use crate::infrastructure::repositories::Repositories;
use crate::infrastructure::repository_impl::application::feature_status::repository::AppRepositoryImpl;

pub mod result;
pub mod service;

pub struct FeatureStatusServices {
    pub get_feature_status: GetWebsiteFeatureStatusService<AppRepositoryImpl>,
}

impl FeatureStatusServices {
    pub fn new(repos: &Repositories) -> Self {
        Self {
            get_feature_status: GetWebsiteFeatureStatusService::new(repos.website.repository.clone()),
        }
    }
}
