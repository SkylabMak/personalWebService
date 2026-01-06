use crate::application::use_cases::profile::profile_use_cases::ProfileUseCases;

#[derive(Clone)]
pub struct AppState {
    pub profile: ProfileUseCases,
}