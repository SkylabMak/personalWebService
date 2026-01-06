pub struct GetLifeStatusInput {
    pub profile_id: String,
}

impl GetLifeStatusInput {
    pub fn new(profile_id: String) -> Self {
        Self { profile_id }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}
