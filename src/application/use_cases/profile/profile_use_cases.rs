use crate::application::services::profile::profile_services::ProfileServices;
use crate::application::use_cases::profile::life_status::LifeStatusUseCases;
use crate::application::use_cases::profile::announce::AnnounceUseCases;

#[derive(Clone)]
pub struct ProfileUseCases {
    pub life_status: LifeStatusUseCases,
    pub announce: AnnounceUseCases,
}

impl ProfileUseCases {
    pub fn new(services: ProfileServices) -> Self {
        let life_status = LifeStatusUseCases::new(services.life_status);
        let announce = AnnounceUseCases::new(services.announce);
        Self {
            life_status,
            announce,
        }
    }
}
