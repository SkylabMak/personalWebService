use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub skill_type: String,
    pub scale_id: String,
    pub scale_value: f64,
    pub logo_url: Option<String>,
    pub description: Option<String>,
}
