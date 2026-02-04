use async_trait::async_trait;
use crate::domain::entities::auth::user::User;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::auth::auth_repository::AuthRepository;

#[derive(Clone)]
pub struct AuthRepositoryImpl {
    mysql: MySqlRepository,
}

impl AuthRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, email, password_hash, role_id, created_at, updated_at
            FROM user
            WHERE username = ?
            "#,
            username
        )
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            role_id: r.role_id,
            created_at: r.created_at.to_string(),
            updated_at: r.updated_at.map(|d| d.to_string()),
        }))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, username, email, password_hash, role_id, created_at, updated_at
            FROM user
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| User {
            id: r.id,
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            role_id: r.role_id,
            created_at: r.created_at.to_string(),
            updated_at: r.updated_at.map(|d| d.to_string()),
        }))
    }
}
