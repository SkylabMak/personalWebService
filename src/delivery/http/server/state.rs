use crate::application::services::services::Services;
use crate::application::use_cases::profile::profile_use_cases::ProfileUseCases;
use crate::application::use_cases::application::WebsiteUseCases;

#[derive(Clone)]
pub struct AppState {
    pub profile: ProfileUseCases,
    pub website: WebsiteUseCases,
}

impl AppState {
    pub fn new(services: Services) -> Self {
        Self {
            profile: ProfileUseCases::new(services.profile),
            website: WebsiteUseCases::new(services.website),
        }
    }
}