use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Social {
    pub id: String,
    pub name: String,
    pub link: String,
    pub logo_url: Option<String>,
}
