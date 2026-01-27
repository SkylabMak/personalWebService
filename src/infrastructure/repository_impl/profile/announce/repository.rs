use async_trait::async_trait;
use crate::domain::entities::profile::announce::announce::Announce;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::announce::announce_repository::AnnounceRepository;

#[derive(Clone)]
pub struct AnnounceRepositoryImpl {
    mysql: MySqlRepository,
}

impl AnnounceRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl AnnounceRepository for AnnounceRepositoryImpl {
    async fn find_active_by_profile_id(
        &self,
        profile_id: &str,
    ) -> Result<Vec<Announce>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                l.id, 
                t.name as type_name, 
                l.title, 
                l.message, 
                l.link_url, 
                l.link_text, 
                t.color_token,
                CAST(l.starts_at AS CHAR) as starts_at,
                CAST(l.ends_at AS CHAR) as ends_at,
                CAST(l.created_at AS CHAR) as "created_at!"
            FROM announce_list l
            JOIN announce_type t ON l.announce_type_id = t.id
            WHERE l.profile_id = ?
            "#,
            profile_id
        )
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| Announce {
                id: r.id,
                announce_type: r.type_name,
                title: r.title,
                message: r.message,
                link_url: r.link_url,
                link_text: r.link_text,
                color_token: r.color_token,
                starts_at: r.starts_at,
                ends_at: r.ends_at,
                created_at: r.created_at,
            })
            .collect())
    }
}
