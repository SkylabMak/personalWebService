use crate::domain::entities::profile::performance::performance::Performance;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::performance::performance_repository::PerformanceRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct PerformanceRepositoryImpl {
    mysql: MySqlRepository,
}

impl PerformanceRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl PerformanceRepository for PerformanceRepositoryImpl {
    async fn create(&self, perf: Performance) -> Result<Performance, RepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO performance (id, profile_id, category_id, visibility_id, title, summary, content_url, content_type, content_preview, start_date, end_date, location, close, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            perf.id, perf.profile_id, perf.category_id, perf.visibility_id,
            perf.title, perf.summary, perf.content_url, perf.content_type,
            perf.content_preview, perf.start_date, perf.end_date,
            perf.location, perf.close as i8, perf.created_at, perf.updated_at
        )
            .execute(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(perf)
    }

    async fn update(&self, perf: Performance) -> Result<Performance, RepositoryError> {
        sqlx::query!(
            r#"
            UPDATE performance
            SET category_id = ?, visibility_id = ?, title = ?, summary = ?, 
                content_url = ?, content_type = ?, content_preview = ?, 
                start_date = ?, end_date = ?, location = ?, close = ?, updated_at = ?
            WHERE id = ?
            "#,
            perf.category_id, perf.visibility_id, perf.title, perf.summary,
            perf.content_url, perf.content_type, perf.content_preview,
            perf.start_date, perf.end_date, perf.location, perf.close as i8,
            perf.updated_at, perf.id
        )
            .execute(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(perf)
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Performance>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                id, 
                profile_id, 
                category_id, 
                visibility_id, 
                title, 
                summary, 
                content_url, 
                content_type, 
                content_preview, 
                start_date, 
                end_date, 
                location, 
                close, 
                created_at, 
                updated_at
            FROM performance
            WHERE id = ?
            "#,
            id
        )
            .fetch_optional(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| Performance {
            id: r.id,
            profile_id: r.profile_id,
            category_id: r.category_id,
            visibility_id: r.visibility_id,
            title: r.title,
            summary: r.summary,
            content_url: r.content_url,
            content_type: r.content_type.unwrap_or_else(|| "markdown".to_string()),
            content_preview: r.content_preview,
            start_date: r.start_date.map(|d| d.to_string()),
            end_date: r.end_date.map(|d| d.to_string()),
            location: r.location,
            close: r.close.unwrap_or(0) != 0, // Convert TINYINT (i8) to bool
            created_at: r.created_at.to_string(),
            updated_at: r.updated_at.map(|d| d.to_string()),
        }))
    }

    async fn delete(&self, id: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM performance
            WHERE id = ?
            "#,
            id
        )
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn track_image_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError> {
        // Using MySQL's ON DUPLICATE KEY UPDATE as specified in the design
        sqlx::query!(
            r#"
            INSERT INTO image_usage (id, image_id, performance_id, usage_count, first_used_at, last_used_at)
            VALUES (UUID(), ?, ?, 1, NOW(), NOW())
            ON DUPLICATE KEY UPDATE
                usage_count = usage_count + 1,
                last_used_at = NOW()
            "#,
            image_id,
            performance_id
        )
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn untrack_image_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError> {
        // Decrement usage or remove if it was the last one
        // First check current usage
        let row = sqlx::query!(
            r#"
            SELECT id, usage_count FROM image_usage
            WHERE image_id = ? AND performance_id = ?
            "#,
            image_id,
            performance_id
        )
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        if let Some(r) = row {
            let current_count = r.usage_count.unwrap_or(0);
            if current_count <= 1 {
                sqlx::query!(
                    r#"
                    DELETE FROM image_usage WHERE id = ?
                    "#,
                    r.id
                )
                .execute(self.mysql.pool())
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            } else {
                sqlx::query!(
                    r#"
                    UPDATE image_usage SET usage_count = usage_count - 1 WHERE id = ?
                    "#,
                    r.id
                )
                .execute(self.mysql.pool())
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            }
        }

        Ok(())
    }

    async fn get_tracked_images(&self, performance_id: &str) -> Result<Vec<String>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT image_id FROM image_usage
            WHERE performance_id = ?
            "#,
            performance_id
        )
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.image_id).collect())
    }

    async fn delete_image_usage_by_performance_id(&self, performance_id: &str) -> Result<(), RepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM image_usage
            WHERE performance_id = ?
            "#,
            performance_id
        )
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
