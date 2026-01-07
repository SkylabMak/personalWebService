use crate::delivery::http::server::server::create_router;
use crate::delivery::http::server::state::AppState;
use crate::config::config::Config;
use crate::infrastructure::infrastructure::Infrastructure;
use crate::application::services::services::Services;

pub struct App;

impl App {
    pub async fn create_router() -> anyhow::Result<axum::Router> {
        let config = Config::from_env()?;

        // 1. Setup Infrastructure
        let infra = Infrastructure::new(&config).await?;

        // 2. Set up Application Services
        let services = Services::new(infra);

        // 3. Setup AppState
        let state = AppState::new(services);

        // 4. Build router
        Ok(create_router(state))
    }
}
