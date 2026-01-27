use crate::domain::entities::application::feature_status::app_feature_status::AppFeatureStatus;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::application::feature_status::app_repository::AppRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct AppRepositoryImpl {
    mysql: MySqlRepository,
}

impl AppRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl AppRepository for AppRepositoryImpl {
    async fn find_feature_statuses_by_website_id(
        &self,
        website_id: &str,
    ) -> Result<Vec<AppFeatureStatus>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT
                f.code as feature_code,
                f.name as feature_name,
                fs.id as status_code,
                fs.name as status_name,
                CAST(fss.updated_at AS CHAR) as "updated_at!",
                fss.note as note,
                CASE WHEN fs.id = 'fs_closed' THEN 1 ELSE 0 END as "is_closed!"
            FROM feature_list fl
            JOIN feature f ON fl.feature_id = f.id
            JOIN feature_status_state fss ON fl.website_id = fss.website_id AND fl.id = fss.feature_list_id
            JOIN feature_status fs ON fss.status_id = fs.id
            WHERE fl.website_id = ?
            "#,
            website_id
        )
            .fetch_all(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| AppFeatureStatus {
            feature_code: r.feature_code,
            feature_name: r.feature_name,
            status_code: r.status_code,
            status_name: r.status_name,
            is_closed: r.is_closed != 0,
            updated_at: r.updated_at,
            note: r.note,
        }).collect())
    }
}
