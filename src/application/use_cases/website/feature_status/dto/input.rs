pub struct GetWebsiteFeatureStatusInput {
    pub website_id: String,
}

impl GetWebsiteFeatureStatusInput {
    pub fn new(website_id: String) -> Self {
        Self { website_id }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.website_id.trim().is_empty() {
            return Err("website_id cannot be empty".to_string());
        }
        Ok(())
    }
}
