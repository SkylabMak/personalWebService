use crate::delivery::http::server::server::create_router;
use crate::delivery::http::server::state::AppState;
use crate::config::config::Config;
use crate::infrastructure::infrastructure::Infrastructure;
use crate::application::services::services::Services;
use crate::application::use_cases::profile::life_status::LifeStatusUseCases;
use crate::application::use_cases::profile::profile_use_cases::ProfileUseCases;

pub struct App;

impl App {
    pub async fn create_router() -> anyhow::Result<axum::Router> {
        let config = Config::from_env()?;

        // 1. Setup Infrastructure
        let infra = Infrastructure::new(&config).await?;

        // 2. Set up Application Services
        let services = Services::new(infra);

        // 3. Setup AppState
        let life_status_use_cases = LifeStatusUseCases::new(services.profile.life_status);
        let profile_use_cases = ProfileUseCases::new(life_status_use_cases);

        let state = AppState {
            profile: profile_use_cases,
        };

        // 4. Build router
        Ok(create_router(state))
    }
}
