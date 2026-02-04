use std::sync::Arc;
use crate::application::services::auth::auth_services::AuthServices;
use crate::application::services::auth::auth_service::AuthService;
use crate::infrastructure::repository_impl::auth::repository::AuthRepositoryImpl;
use crate::infrastructure::repository_impl::auth::refresh_token_repository_impl::RefreshTokenRepositoryImpl;

#[derive(Clone)]
pub struct AuthUseCases {
    pub auth: Arc<AuthService<AuthRepositoryImpl, RefreshTokenRepositoryImpl>>,
}

impl AuthUseCases {
    pub fn new(services: AuthServices) -> Self {
        Self {
            auth: Arc::new(services.auth),
        }
    }
}
