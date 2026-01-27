use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Announce {
    pub id: String,
    pub announce_type: String,
    pub title: Option<String>,
    pub message: Option<String>,
    pub link_url: Option<String>,
    pub link_text: Option<String>,
    pub color_token: Option<String>,
    pub starts_at: Option<String>,
    pub ends_at: Option<String>,
    pub created_at: String,
}

impl Announce {
    pub fn new(
        id: String,
        announce_type: String,
        title: Option<String>,
        message: Option<String>,
        link_url: Option<String>,
        link_text: Option<String>,
        color_token: Option<String>,
        starts_at: Option<String>,
        ends_at: Option<String>,
        created_at: String,
    ) -> Self {
        Self {
            id,
            announce_type,
            title,
            message,
            link_url,
            link_text,
            color_token,
            starts_at,
            ends_at,
            created_at,
        }
    }
}
