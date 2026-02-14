use async_trait::async_trait;
use crate::domain::entities::profile::profile::Profile;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::profile_repository::ProfileRepository;

#[derive(sqlx::FromRow)]
struct ProfileRecord {
    id: String,
    current_status_id: Option<String>,
}

#[derive(Clone)]
pub struct ProfileRepositoryImpl {
    mysql: MySqlRepository,
}

impl ProfileRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl ProfileRepository for ProfileRepositoryImpl {
    async fn find_by_id(&self, id: &str) -> Result<Option<Profile>, RepositoryError> {
        let row = sqlx::query_as::<_, ProfileRecord>(
            r#"
            SELECT id, current_status_id
            FROM profile
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| Profile {
            id: r.id,
            current_status_id: r.current_status_id,
        }))
    }
}
