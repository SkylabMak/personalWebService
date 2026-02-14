pub struct GetProfileInput {
    pub profile_id: String,
}

impl GetProfileInput {
    pub fn new(profile_id: String) -> Self {
        Self { profile_id }
    }
}
