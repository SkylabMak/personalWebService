use crate::application::services::profile::profile_services::ProfileServices;
use crate::infrastructure::infrastructure::Infrastructure;

pub struct Services {
    pub profile: ProfileServices,
}

impl Services {
    pub fn new(infra: Infrastructure) -> Self {
        Self {
            profile: ProfileServices::new(&infra.repositories),
        }
    }
}
