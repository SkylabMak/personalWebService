use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Profile {
    pub id: String,
    pub current_status_id: Option<String>,
}
