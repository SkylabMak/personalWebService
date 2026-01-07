pub struct WebsiteFeatureStatus {
    pub feature_code: String,
    pub feature_name: String,
    pub status_name: String,
    pub is_closed: bool,
    pub updated_at: String, // Or use a Date type if available
    pub note: Option<String>,
}

impl WebsiteFeatureStatus {
    pub fn new(
        feature_code: String,
        feature_name: String,
        status_name: String,
        is_closed: bool,
        updated_at: String,
        note: Option<String>,
    ) -> Self {
        Self {
            feature_code,
            feature_name,
            status_name,
            is_closed,
            updated_at,
            note,
        }
    }
}
