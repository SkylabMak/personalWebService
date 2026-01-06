use crate::application::use_cases::profile::life_status::LifeStatusUseCases;

#[derive(Clone)]
pub struct ProfileUseCases {
    pub life_status: LifeStatusUseCases,
}

impl ProfileUseCases {
    pub fn new(life_status: LifeStatusUseCases) -> Self {
        Self { life_status }
    }
}
