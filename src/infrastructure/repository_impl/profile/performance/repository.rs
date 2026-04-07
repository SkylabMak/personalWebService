use crate::domain::entities::profile::performance::performance::Performance;
use crate::domain::entities::profile::image::image::Image;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::performance::performance_repository::PerformanceRepository;
use async_trait::async_trait;
use sqlx::Row;

#[derive(sqlx::FromRow)]
struct PerformanceRecord {
    id: String,
    profile_id: String,
    category_id: String,
    visibility_id: String,
    title: String,
    summary: Option<String>,
    content_url: Option<String>,
    content_type: Option<String>,
    content_preview: Option<String>,
    start_date: Option<sqlx::types::chrono::NaiveDate>,
    end_date: Option<sqlx::types::chrono::NaiveDate>,
    location: Option<String>,
    close: i8,
    created_at: sqlx::types::chrono::NaiveDate,
    updated_at: Option<sqlx::types::chrono::NaiveDate>,
}

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
        let row = sqlx::query_as::<_, PerformanceRecord>(
            r#"
            SELECT 
                id, profile_id, category_id, visibility_id, title, summary, 
                content_url, content_type, content_preview, start_date, 
                end_date, location, close, created_at, updated_at
            FROM performance
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(record_to_performance))
    }

    async fn find_by_profile_id(
        &self,
        profile_id: &str,
        visibility_id: Option<&str>,
    ) -> Result<Vec<Performance>, RepositoryError> {
        let query = if visibility_id.is_some() {
            r#"
            SELECT 
                id, profile_id, category_id, visibility_id, title, summary, 
                content_url, content_type, content_preview, start_date, 
                end_date, location, close, created_at, updated_at
            FROM performance
            WHERE profile_id = ? AND visibility_id = ?
            ORDER BY created_at DESC
            "#
        } else {
            r#"
            SELECT 
                id, profile_id, category_id, visibility_id, title, summary, 
                content_url, content_type, content_preview, start_date, 
                end_date, location, close, created_at, updated_at
            FROM performance
            WHERE profile_id = ?
            ORDER BY created_at DESC
            "#
        };

        let mut q = sqlx::query_as::<_, PerformanceRecord>(query).bind(profile_id);
        if let Some(vid) = visibility_id {
            q = q.bind(vid);
        }

        let rows = q
            .fetch_all(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(record_to_performance)
            .collect())
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

    async fn sync_image_usage(
        &self,
        performance_id: &str,
        current_image_ids: &[String],
    ) -> Result<(), RepositoryError> {
        let pool = self.mysql.pool();

        if current_image_ids.is_empty() {
            sqlx::query!(
                "DELETE FROM image_usage WHERE performance_id = ?",
                performance_id
            )
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
            return Ok(());
        }

        // Bulk UPSERT
        // Group by image_id to get correct counts for each performance
        use std::collections::HashMap;
        let mut image_counts = HashMap::new();
        for image_id in current_image_ids {
            *image_counts.entry(image_id).or_insert(0) += 1;
        }

        for (image_id, count) in image_counts {
            sqlx::query!(
                r#"
                INSERT INTO image_usage (id, image_id, performance_id, usage_count, first_used_at, last_used_at)
                VALUES (UUID(), ?, ?, ?, NOW(), NOW())
                ON DUPLICATE KEY UPDATE
                    usage_count = VALUES(usage_count),
                    last_used_at = NOW()
                "#,
                image_id,
                performance_id,
                count
            )
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        // Delete Orphans
        let mut query_builder = sqlx::QueryBuilder::new("DELETE FROM image_usage WHERE performance_id = ");
        query_builder.push_bind(performance_id);
        query_builder.push(" AND image_id NOT IN (");
        let mut separated = query_builder.separated(", ");
        for id in current_image_ids {
            separated.push_bind(id);
        }
        separated.push_unseparated(")");

        query_builder.build()
            .execute(pool)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
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

    async fn find_images_by_performance_id(
        &self,
        performance_id: &str,
    ) -> Result<Vec<Image>, RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                i.id, i.profile_id, i.filename, i.original_filename, i.storage_url,
                i.file_size, i.width, i.height, i.mime_type, i.alt_text, i.caption, i.created_at
            FROM image i
            INNER JOIN image_usage iu ON i.id = iu.image_id
            WHERE iu.performance_id = ?
            "#
        )
        .bind(performance_id)
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut images = Vec::new();
        for r in rows {
            images.push(Image {
                id: r.get("id"),
                profile_id: r.get("profile_id"),
                filename: r.get("filename"),
                original_filename: r.get("original_filename"),
                storage_url: r.get("storage_url"),
                file_size: r.get("file_size"),
                width: r.get("width"),
                height: r.get("height"),
                mime_type: r.get("mime_type"),
                alt_text: r.get("alt_text"),
                caption: r.get("caption"),
                created_at: format!("{:?}", r.get_unchecked::<sqlx::types::chrono::NaiveDateTime, _>("created_at")),
            });
        }

        Ok(images)
    }
}

fn record_to_performance(r: PerformanceRecord) -> Performance {
    Performance {
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
        close: r.close != 0,
        created_at: r.created_at.to_string(),
        updated_at: r.updated_at.map(|d| d.to_string()),
    }
}
