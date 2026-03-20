use async_trait::async_trait;
use crate::domain::entities::profile::profile::Profile;
use crate::domain::entities::profile::life_status::life_status::LifeStatus;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::profile_repository::ProfileRepository;

#[derive(sqlx::FromRow)]
struct ProfileRecord {
    id: String,
    user_id: String,
    display_name: String,
    headline: Option<String>,
    bio: Option<String>,
    avatar_url: Option<String>,
    contact_email: String,
    created_at: chrono::NaiveDate,
    updated_at: Option<chrono::NaiveDate>,
    current_status_id: Option<String>,
    status_name: Option<String>,
    status_description: Option<String>,
    status_color_token: Option<String>,
}

#[derive(Clone)]
pub struct ProfileDataRepositoryImpl {
    mysql: MySqlRepository,
}

impl ProfileDataRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl ProfileRepository for ProfileDataRepositoryImpl {
    async fn find_by_id(&self, id: &str) -> Result<Option<Profile>, RepositoryError> {
        let row = sqlx::query_as::<_, ProfileRecord>(
            r#"
            SELECT 
                p.id, p.user_id, p.display_name, p.headline, p.bio, p.avatar_url, p.contact_email, p.created_at, p.updated_at, p.current_status_id,
                ls.name as status_name, ls.description as status_description, ls.color_token as status_color_token
            FROM profile p
            LEFT JOIN life_status ls ON p.current_status_id = ls.id
            WHERE p.id = ?
            "#
        )
        .bind(id)
        .fetch_optional(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| {
            let current_status = match (r.status_name, r.status_color_token) {
                (Some(name), Some(color_token)) => Some(LifeStatus {
                    name,
                    description: r.status_description,
                    color_token,
                }),
                _ => None,
            };

            Profile {
                id: r.id,
                user_id: r.user_id,
                display_name: r.display_name,
                headline: r.headline,
                bio: r.bio,
                avatar_url: r.avatar_url,
                contact_email: Some(r.contact_email),
                created_at: r.created_at.to_string(),
                updated_at: r.updated_at.map(|d| d.to_string()),
                current_status_id: r.current_status_id,
                current_status,
            }
        }))
    }
}
