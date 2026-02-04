use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use crate::domain::entities::auth::claims::Claims;
use chrono::{Utc, Duration};
use uuid::Uuid;
use sha2::{Sha256, Digest};
use anyhow::Result;

pub struct JwtService {
    secret: String,
    access_expiry: u64,
}

impl JwtService {
    pub fn new(secret: String, access_expiry: u64) -> Self {
        Self { secret, access_expiry }
    }

    pub fn generate_access_token(&self, user_id: &str, role: &str) -> Result<String> {
        let iat = Utc::now().timestamp() as usize;
        let exp = (Utc::now() + Duration::seconds(self.access_expiry as i64)).timestamp() as usize;
        let jti = Uuid::new_v4().to_string();

        let claims = Claims {
            jti,
            sub: user_id.to_string(),
            role: role.to_string(),
            exp,
            iat,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        ).map_err(|e| anyhow::anyhow!("Token generation failed: {}", e))
    }

    pub fn validate_access_token(&self, token: &str) -> Result<Claims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &validation,
        ).map_err(|e| anyhow::anyhow!("Token validation failed: {}", e))?;

        Ok(token_data.claims)
    }

    pub fn generate_refresh_token(&self) -> String {
        Uuid::new_v4().to_string()
    }

    pub fn hash_token(&self, token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}
