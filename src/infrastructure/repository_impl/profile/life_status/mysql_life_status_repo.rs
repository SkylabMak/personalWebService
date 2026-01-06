use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::life_status::life_status_repository::{LifeStatusData, LifeStatusRepository};
use async_trait::async_trait;

#[derive(Clone)]
pub struct MySqlLifeStatusRepository {
    mysql: MySqlRepository,
}

impl MySqlLifeStatusRepository {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}


#[async_trait]
impl LifeStatusRepository for MySqlLifeStatusRepository {
    async fn find_current_by_profile_id(
        &self,
        profile_id: &str,
    ) -> Result<Option<LifeStatusData>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT
                ls.id,
                ls.name,
                ls.description,
                ls.color_token
            FROM profile p
            JOIN life_status ls
              ON p.current_status_id = ls.id
            WHERE p.id = ?
            "#,
            profile_id
        )
            .fetch_optional(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // println!("profile_id is, {}!", profile_id);
        Ok(row.map(|r| LifeStatusData {
            name: r.name,
            description: r.description,
            color_token: r.color_token,
        }))
    }
}
