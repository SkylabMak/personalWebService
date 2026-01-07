use crate::application::services::profile::profile_services::ProfileServices;
use crate::application::services::website::website_services::WebsiteServices;
use crate::infrastructure::infrastructure::Infrastructure;

pub struct Services {
    pub profile: ProfileServices,
    pub website: WebsiteServices,
}

impl Services {
    pub fn new(infra: Infrastructure) -> Self {
        Self {
            profile: ProfileServices::new(&infra.repositories),
            website: WebsiteServices::new(&infra.repositories),
        }
    }
}
