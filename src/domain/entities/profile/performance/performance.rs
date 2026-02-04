use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Performance {
    pub id: String,
    pub profile_id: String,
    pub category_id: String,
    pub visibility_id: String,
    pub title: String,
    pub summary: Option<String>,

    /// The URL pointing to the markdown file in GCS: 
    /// "personal-website_storage/performance_content/{profile_id}/{performance_id}/content.md"
    pub content_url: Option<String>,

    /// The format of the content, defaults to "markdown"
    pub content_type: String,

    /// A plain-text snippet (approx. 500 chars) for search/previews
    pub content_preview: Option<String>,

    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub location: Option<String>,
    pub close: bool,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl Performance {
    pub fn new(
        id: String,
        profile_id: String,
        category_id: String,
        visibility_id: String,
        title: String,
        summary: Option<String>,
        content_url: Option<String>,
        content_type: Option<String>, // Option here to allow defaulting in the constructor
        content_preview: Option<String>,
        start_date: Option<String>,
        end_date: Option<String>,
        location: Option<String>,
        close: bool,
        created_at: String,
        updated_at: Option<String>,
    ) -> Self {
        Self {
            id,
            profile_id,
            category_id,
            visibility_id,
            title,
            summary,
            content_url,
            content_type: content_type.unwrap_or_else(|| "markdown".to_string()),
            content_preview,
            start_date,
            end_date,
            location,
            close,
            created_at,
            updated_at,
        }
    }
}