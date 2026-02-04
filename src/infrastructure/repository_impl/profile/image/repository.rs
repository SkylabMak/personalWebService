use async_trait::async_trait;
use sqlx::{Row};
use crate::domain::entities::profile::image::image::Image;
use crate::domain::entities::profile::image::image_usage::ImageUsageInfo;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::image::image_repository::ImageRepository;

#[derive(Clone)]
pub struct ImageRepositoryImpl {
    mysql: MySqlRepository,
}

impl ImageRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl ImageRepository for ImageRepositoryImpl {
    async fn find_all_by_profile_id(
        &self,
        profile_id: &str,
        search: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<(Vec<(Image, i32)>, usize), RepositoryError> {
        let mut query_str = String::from(
            r#"
            SELECT 
                i.id, i.profile_id, i.filename, i.original_filename, i.storage_url,
                i.file_size, i.width, i.height, i.mime_type, i.alt_text, i.caption, i.created_at,
                CAST(COALESCE(SUM(iu.usage_count), 0) AS SIGNED) as total_usage
            FROM image i
            LEFT JOIN image_usage iu ON i.id = iu.image_id
            WHERE i.profile_id = ?
            "#,
        );

        if let Some(ref s) = search {
            if !s.is_empty() {
                query_str.push_str(" AND (i.filename LIKE ? OR i.alt_text LIKE ? OR i.caption LIKE ?)");
            }
        }

        query_str.push_str(" GROUP BY i.id ORDER BY i.created_at DESC LIMIT ? OFFSET ?");

        let mut query = sqlx::query(&query_str).bind(profile_id);

        if let Some(ref s) = search {
            if !s.is_empty() {
                let s_bind = format!("%{}%", s);
                query = query.bind(s_bind.clone()).bind(s_bind.clone()).bind(s_bind);
            }
        }

        let rows = query.bind(limit).bind(offset)
            .fetch_all(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let images = rows.into_iter().map(|r| {
            let img = Image {
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
            };
            let usage: i64 = r.get("total_usage");
            (img, usage as i32)
        }).collect();

        // Count total
        let mut count_query_str = String::from("SELECT COUNT(*) FROM image WHERE profile_id = ?");
        if let Some(ref s) = search {
             if !s.is_empty() {
                count_query_str.push_str(" AND (filename LIKE ? OR alt_text LIKE ? OR caption LIKE ?)");
            }
        }
        
        let mut count_query = sqlx::query_scalar::<_, i64>(&count_query_str).bind(profile_id);
        if let Some(ref s) = search {
            if !s.is_empty() {
                let s_bind = format!("%{}%", s);
                count_query = count_query.bind(s_bind.clone()).bind(s_bind.clone()).bind(s_bind);
            }
        }

        let total = count_query.fetch_one(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok((images, total as usize))
    }

    async fn find_by_id_and_profile_id(
        &self,
        id: &str,
        profile_id: &str,
    ) -> Result<Option<(Image, i32)>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT 
                i.id, i.profile_id, i.filename, i.original_filename, i.storage_url,
                i.file_size, i.width, i.height, i.mime_type, i.alt_text, i.caption, i.created_at,
                CAST(COALESCE(SUM(iu.usage_count), 0) AS SIGNED) as total_usage
            FROM image i
            LEFT JOIN image_usage iu ON i.id = iu.image_id
            WHERE i.id = ? AND i.profile_id = ?
            GROUP BY i.id
            "#
        )
        .bind(id)
        .bind(profile_id)
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| {
            let img = Image {
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
            };
            let usage: i64 = r.get("total_usage");
            (img, usage as i32)
        }))
    }

    async fn find_usage_by_image_id(
        &self,
        image_id: &str,
        profile_id: &str,
    ) -> Result<Vec<ImageUsageInfo>, RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT 
                iu.performance_id,
                p.title,
                iu.usage_count,
                iu.first_used_at,
                iu.last_used_at
            FROM image_usage iu
            JOIN performance p ON iu.performance_id = p.id
            JOIN image i ON iu.image_id = i.id
            WHERE iu.image_id = ? AND i.profile_id = ?
            "#
        )
        .bind(image_id)
        .bind(profile_id)
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| {
            ImageUsageInfo {
                performance_id: r.get("performance_id"),
                title: r.get("title"),
                usage_count: r.get("usage_count"),
                first_used_at: format!("{:?}", r.get_unchecked::<sqlx::types::chrono::NaiveDateTime, _>("first_used_at")),
                last_used_at: format!("{:?}", r.get_unchecked::<sqlx::types::chrono::NaiveDateTime, _>("last_used_at")),
            }
        }).collect())
    }

    async fn create(&self, image: Image) -> Result<(), RepositoryError> {
        // Save to MySQL
        let created_at = sqlx::types::chrono::NaiveDateTime::parse_from_str(&image.created_at, "%Y-%m-%dT%H:%M:%SZ")
            .unwrap_or_else(|_| sqlx::types::chrono::Utc::now().naive_utc());

        sqlx::query(
            r#"
            INSERT INTO image (
                id, profile_id, filename, original_filename, storage_url,
                file_size, width, height, mime_type, alt_text, caption, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(image.id)
        .bind(image.profile_id)
        .bind(image.filename)
        .bind(image.original_filename)
        .bind(image.storage_url)
        .bind(image.file_size)
        .bind(image.width)
        .bind(image.height)
        .bind(image.mime_type)
        .bind(image.alt_text)
        .bind(image.caption)
        .bind(created_at)
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn track_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError> {
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

    async fn untrack_usage(
        &self,
        image_id: &str,
        performance_id: &str,
    ) -> Result<(), RepositoryError> {
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

    async fn update_metadata(
        &self,
        id: &str,
        profile_id: &str,
        alt_text: Option<String>,
        caption: Option<String>,
    ) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            UPDATE image 
            SET alt_text = ?, caption = ?
            WHERE id = ? AND profile_id = ?
            "#
        )
        .bind(alt_text)
        .bind(caption)
        .bind(id)
        .bind(profile_id)
        .execute(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &str, profile_id: &str) -> Result<(), RepositoryError> {
        sqlx::query("DELETE FROM image WHERE id = ? AND profile_id = ?")
            .bind(id)
            .bind(profile_id)
            .execute(self.mysql.pool())
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn force_delete(&self, id: &str, profile_id: &str) -> Result<(), RepositoryError> {
        let mut tx = self.mysql.pool().begin().await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // 1. Delete from image_usage
        sqlx::query("DELETE FROM image_usage WHERE image_id = ?")
            .bind(id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // 2. Delete from image
        sqlx::query("DELETE FROM image WHERE id = ? AND profile_id = ?")
            .bind(id)
            .bind(profile_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        tx.commit().await
            .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_unused_by_profile_id(
        &self,
        profile_id: &str,
        days_old: i32,
    ) -> Result<Vec<Image>, RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT
                i.id, i.profile_id, i.filename, i.original_filename, i.storage_url,
                i.file_size, i.width, i.height, i.mime_type, i.alt_text, i.caption, i.created_at
            FROM image i
            LEFT JOIN image_usage iu ON i.id = iu.image_id
            WHERE i.profile_id = ?
            GROUP BY i.id
            HAVING COALESCE(SUM(iu.usage_count), 0) = 0
               AND DATEDIFF(NOW(), i.created_at) >= ?
            "#
        )
        .bind(profile_id)
        .bind(days_old)
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        Ok(rows.into_iter().map(|r| {
            Image {
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
            }
        }).collect())
    }

    async fn delete_unused_by_profile_id(
        &self,
        profile_id: &str,
        days_old: i32,
    ) -> Result<(i64, i64), RepositoryError> {
        // This is tricky because we want to know what we deleted.
        // For simplicity, let's just find them first and then delete.
        // But the requirement says return (count, size).
        
        let unused = self.find_unused_by_profile_id(profile_id, days_old).await?;
        let count = unused.len() as i64;
        let total_size: i32 = unused.iter().map(|img| img.file_size).sum();

        if count > 0 {
            let ids: Vec<String> = unused.into_iter().map(|img| img.id).collect();
            // MySQL doesn't support DELETE with JOIN/HAVING directly as easily as SELECT.
            // Using IN (...)
            let query_str = format!("DELETE FROM image WHERE id IN ({})", ids.iter().map(|_| "?").collect::<Vec<_>>().join(","));
            let mut query = sqlx::query(&query_str);
            for id in ids {
                query = query.bind(id);
            }
            query.execute(self.mysql.pool())
                .await
                .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
        }

        Ok((count, total_size as i64))
    }
}
