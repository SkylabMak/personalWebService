use crate::application::services::auth::auth_service::AuthService;
use crate::application::services::auth::jwt_service::JwtService;
use crate::application::services::auth::password_service::PasswordService;
use crate::infrastructure::repositories::Repositories;
use crate::infrastructure::repository_impl::auth::repository::AuthRepositoryImpl;
use crate::infrastructure::repository_impl::auth::refresh_token_repository_impl::RefreshTokenRepositoryImpl;
use crate::config::config::Config;

pub struct AuthServices {
    pub auth: AuthService<AuthRepositoryImpl, RefreshTokenRepositoryImpl>,
}

impl AuthServices {
    pub fn new(repos: &Repositories, config: &Config) -> Self {
        let jwt_service = JwtService::new(
            config.jwt_secret.clone(),
            config.jwt_access_expiry,
        );

        let password_service = PasswordService::new(
            config.argon2_salt.clone(),
            config.argon2_memory_cost,
            config.argon2_iterations,
            config.argon2_parallelism,
        );

        Self {
            auth: AuthService::new(
                repos.auth.auth.clone(),
                repos.auth.refresh_token.clone(),
                jwt_service,
                password_service,
                config.jwt_refresh_expiry,
                config.jwt_access_expiry,
            ),
        }
    }
}
