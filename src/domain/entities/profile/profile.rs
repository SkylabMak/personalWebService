use serde::Serialize;
use crate::domain::entities::profile::life_status::life_status::LifeStatus;

#[derive(Debug, Clone, Serialize)]
pub struct Profile {
    pub id: String,
    pub user_id: String,
    pub display_name: String,
    pub headline: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub contact_email: Option<String>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub current_status_id: Option<String>,
    pub current_status: Option<LifeStatus>,
}
