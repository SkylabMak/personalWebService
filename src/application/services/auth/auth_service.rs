use chrono::{Utc, Duration};
use crate::application::errors::{ApplicationError, MapToApplicationError};
use crate::application::services::auth::jwt_service::JwtService;
use crate::application::services::auth::password_service::PasswordService;
use crate::application::services::auth::result::{LoginResult, AuthUserResponse, RefreshResult, LogoutResult, MeResult};
use crate::domain::entities::auth::refresh_token::RefreshToken;
use crate::interface_adapters::gateways::repositories::auth::auth_repository::AuthRepository;
use crate::interface_adapters::gateways::repositories::auth::refresh_token_repository::RefreshTokenRepository;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService<AR, RR>
where
    AR: AuthRepository,
    RR: RefreshTokenRepository,
{
    pub auth_repo: AR,
    pub refresh_token_repo: RR,
    pub jwt_service: std::sync::Arc<JwtService>,
    pub password_service: std::sync::Arc<PasswordService>,
    pub refresh_expiry: u64,
    pub access_expiry: u64,
}

impl<AR, RR> AuthService<AR, RR>
where
    AR: AuthRepository + Clone,
    RR: RefreshTokenRepository + Clone,
{
    pub fn new(
        auth_repo: AR,
        refresh_token_repo: RR,
        jwt_service: JwtService,
        password_service: PasswordService,
        refresh_expiry: u64,
        access_expiry: u64,
    ) -> Self {
        Self {
            auth_repo,
            refresh_token_repo,
            jwt_service: std::sync::Arc::new(jwt_service),
            password_service: std::sync::Arc::new(password_service),
            refresh_expiry,
            access_expiry,
        }
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResult, ApplicationError> {
        let user = self.auth_repo
            .find_by_username(username)
            .await
            .map_app_err("Failed to fetch user")?
            .ok_or(ApplicationError::Unauthorized)?;

        // Simple check (in production use bcrypt or similar)
        if !self.password_service.verify_password(password, &user.password_hash)? {
             return Err(ApplicationError::Unauthorized);
        }

        let access_token = self.jwt_service
            .generate_access_token(&user.id, &user.role_id)
            .map_err(|e| ApplicationError::Internal { message: e.to_string() })?;

        let refresh_token = self.jwt_service.generate_refresh_token();
        let refresh_token_hash = self.jwt_service.hash_token(&refresh_token);

        let expires_at = Utc::now() + Duration::seconds(self.refresh_expiry as i64);
        
        let rt_entity = RefreshToken {
            id: Uuid::new_v4().to_string(),
            user_id: user.id.clone(),
            token_hash: refresh_token_hash,
            expires_at,
            created_at: Utc::now(),
            last_used_at: None,
            device_info: None,
        };

        self.refresh_token_repo
            .save(&rt_entity)
            .await
            .map_app_err("Failed to save refresh token")?;

        Ok(LoginResult {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_expiry,
            refresh_token_expires_in: self.refresh_expiry,
            user: AuthUserResponse {
                id: user.id,
                username: user.username,
                role: user.role_id,
            },
        })
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<RefreshResult, ApplicationError> {
        let hash = self.jwt_service.hash_token(refresh_token);
        
        let rt = self.refresh_token_repo
            .find_by_token_hash(&hash)
            .await
            .map_app_err("Failed to find refresh token")?
            .ok_or(ApplicationError::Unauthorized)?;

        if rt.expires_at < Utc::now() {
            return Err(ApplicationError::Unauthorized);
        }

        let user = self.auth_repo
            .find_by_id(&rt.user_id)
            .await
            .map_app_err("Failed to fetch user")?
            .ok_or(ApplicationError::Unauthorized)?;

        let access_token = self.jwt_service
            .generate_access_token(&user.id, &user.role_id)
            .map_err(|e| ApplicationError::Internal { message: e.to_string() })?;

        self.refresh_token_repo
            .update_last_used(&hash)
            .await
            .map_app_err("Failed to update refresh token usage")?;

        Ok(RefreshResult {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in: self.access_expiry,
        })
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<LogoutResult, ApplicationError> {
        let hash = self.jwt_service.hash_token(refresh_token);
        
        self.refresh_token_repo
            .delete_by_token_hash(&hash)
            .await
            .map_app_err("Failed to delete refresh token")?;

        Ok(LogoutResult {
            message: "Logged out successfully".to_string(),
        })
    }

    pub async fn me(&self, user_id: &str) -> Result<MeResult, ApplicationError> {
        let user = self.auth_repo
            .find_by_id(user_id)
            .await
            .map_app_err("Failed to fetch user")?
            .ok_or(ApplicationError::NotFound { resource: "User", identifier: user_id.to_string() })?;

        Ok(MeResult {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role_id,
        })
    }
}
