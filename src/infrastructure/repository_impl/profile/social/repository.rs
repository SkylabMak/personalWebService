use async_trait::async_trait;
use crate::domain::entities::profile::social::social::Social;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::social::social_repository::SocialRepository;

#[derive(sqlx::FromRow)]
struct SocialRecord {
    id: String,
    name: String,
    link: String,
    logo_url: Option<String>,
}

#[derive(Clone)]
pub struct SocialRepositoryImpl {
    mysql: MySqlRepository,
}

impl SocialRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl SocialRepository for SocialRepositoryImpl {
    async fn find_by_profile_id(&self, profile_id: &str) -> Result<Vec<Social>, RepositoryError> {
        let rows = sqlx::query_as::<_, SocialRecord>(
            r#"
            SELECT 
                s.id, s.name, sl.link, s.logo_url
            FROM social_list sl
            JOIN social s ON sl.social_id = s.id
            WHERE sl.profile_id = ?
            "#
        )
        .bind(profile_id)
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| Social {
            id: r.id,
            name: r.name,
            link: r.link,
            logo_url: r.logo_url,
        }).collect())
    }
}
