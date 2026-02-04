use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::entities::auth::refresh_token::RefreshToken;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::auth::refresh_token_repository::RefreshTokenRepository;

#[derive(Clone)]
pub struct RefreshTokenRepositoryImpl {
    mysql: MySqlRepository,
}

impl RefreshTokenRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenRepositoryImpl {
    async fn save(&self, token: &RefreshToken) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO refresh_token (id, user_id, token_hash, expires_at, created_at, last_used_at, device_info)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            token.id,
            token.user_id,
            token.token_hash,
            token.expires_at.naive_utc(),
            token.created_at.naive_utc(),
            token.last_used_at.map(|d| d.naive_utc()),
            token.device_info
        )
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_token_hash(&self, hash: &str) -> Result<Option<RefreshToken>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, user_id, token_hash, expires_at, created_at, last_used_at, device_info
            FROM refresh_token
            WHERE token_hash = ?
            "#,
            hash
        )
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| RefreshToken {
            id: r.id,
            user_id: r.user_id,
            token_hash: r.token_hash,
            expires_at: DateTime::<Utc>::from_naive_utc_and_offset(r.expires_at, Utc),
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(r.created_at, Utc),
            last_used_at: r.last_used_at.map(|d| DateTime::<Utc>::from_naive_utc_and_offset(d, Utc)),
            device_info: r.device_info,
        }))
    }

    async fn delete_by_token_hash(&self, hash: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM refresh_token
            WHERE token_hash = ?
            "#,
            hash
        )
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update_last_used(&self, hash: &str) -> Result<(), RepositoryError> {
        let now = Utc::now();
        sqlx::query!(
            r#"
            UPDATE refresh_token
            SET last_used_at = ?
            WHERE token_hash = ?
            "#,
            now.naive_utc(),
            hash
        )
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
