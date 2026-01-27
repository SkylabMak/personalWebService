pub struct GetAnnounceListInput {
    pub profile_id: String,
}

impl GetAnnounceListInput {
    pub fn new(profile_id: String) -> Self {
        Self { profile_id }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.is_empty() {
            return Err("Profile ID cannot be empty".to_string());
        }
        Ok(())
    }
}
