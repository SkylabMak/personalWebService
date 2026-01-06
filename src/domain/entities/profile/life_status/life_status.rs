pub struct LifeStatus {
    pub name: String,
    pub description: Option<String>,
    pub color_token: String,
}

impl LifeStatus {
    pub fn new(name: String, description: Option<String>, color_token: String) -> Self {
        Self {
            name,
            description,
            color_token,
        }
    }
}
