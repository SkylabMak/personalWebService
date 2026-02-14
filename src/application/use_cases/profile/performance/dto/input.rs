pub struct CreatePerformanceInput {
    pub profile_id: String,
    pub category_id: String,
    pub visibility_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub location: Option<String>,
}

impl CreatePerformanceInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        if self.title.trim().is_empty() {
            return Err("title cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct UpdatePerformanceInput {
    pub id: String,
    pub profile_id: String,
    pub category_id: String,
    pub visibility_id: String,
    pub title: String,
    pub summary: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub location: Option<String>,
    pub close: bool,
}

impl UpdatePerformanceInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("id cannot be empty".to_string());
        }
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        if self.title.trim().is_empty() {
            return Err("title cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct DeletePerformanceInput {
    pub id: String,
    pub profile_id: String,
}

impl DeletePerformanceInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("id cannot be empty".to_string());
        }
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct UpdatePerformanceContentInput {
    pub performance_id: String,
    pub profile_id: String,
    pub content_markdown: String,
}

impl UpdatePerformanceContentInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.performance_id.trim().is_empty() {
            return Err("performance_id cannot be empty".to_string());
        }
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct GetPerformanceContentInput {
    pub performance_id: String,
    pub profile_id: String,
}

pub struct ListPerformancesInput {
    pub profile_id: String,
    pub visibility_id: Option<String>,
}

impl ListPerformancesInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}
