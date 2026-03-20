use async_trait::async_trait;
use crate::domain::entities::profile::skill::skill::Skill;
use crate::infrastructure::db::mysql::common::mysql_repository::MySqlRepository;
use crate::interface_adapters::gateways::common::repository_error::RepositoryError;
use crate::interface_adapters::gateways::repositories::profile::skill::skill_repository::SkillRepository;

#[derive(sqlx::FromRow)]
struct SkillRecord {
    id: String,
    name: String,
    skill_type: String,
    scale_id: String,
    scale_value: f64,
    logo_url: Option<String>,
    description: Option<String>,
}

#[derive(Clone)]
pub struct SkillRepositoryImpl {
    mysql: MySqlRepository,
}

impl SkillRepositoryImpl {
    pub fn new(mysql: MySqlRepository) -> Self {
        Self { mysql }
    }
}

#[async_trait]
impl SkillRepository for SkillRepositoryImpl {
    async fn find_by_profile_id(&self, profile_id: &str) -> Result<Vec<Skill>, RepositoryError> {
        let rows = sqlx::query_as::<_, SkillRecord>(
            r#"
            SELECT 
                s.id, s.name, st.name as skill_type, s.scale_id, CAST(sl.scale_value AS DOUBLE) as scale_value, s.logo_url, sl.description
            FROM skill_list sl
            JOIN skill s ON sl.skill_id = s.id
            JOIN skill_type st ON s.skill_type_id = st.id
            WHERE sl.profile_id = ?
            "#
        )
        .bind(profile_id)
        .fetch_all(self.mysql.pool())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows.into_iter().map(|r| Skill {
            id: r.id,
            name: r.name,
            skill_type: r.skill_type,
            scale_id: r.scale_id,
            scale_value: r.scale_value,
            logo_url: r.logo_url,
            description: r.description,
        }).collect())
    }
}
