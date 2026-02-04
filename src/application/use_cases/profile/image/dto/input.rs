pub struct GetImagesInput {
    pub profile_id: String,
    pub search: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

impl GetImagesInput {
    pub fn new(profile_id: String, search: Option<String>, limit: Option<i32>, offset: Option<i32>) -> Self {
        Self { profile_id, search, limit, offset }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct GetImageInput {
    pub id: String,
    pub profile_id: String,
}

impl GetImageInput {
    pub fn new(id: String, profile_id: String) -> Self {
        Self { id, profile_id }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("image id cannot be empty".to_string());
        }
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct CreateImageInput {
    pub profile_id: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub image_bytes: Vec<u8>,
}

impl CreateImageInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        if self.original_filename.trim().is_empty() {
            return Err("original_filename cannot be empty".to_string());
        }
        if self.image_bytes.is_empty() {
            return Err("image_bytes cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct UpdateImageMetadataInput {
    pub id: String,
    pub profile_id: String,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
}

impl UpdateImageMetadataInput {
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

pub struct DeleteImageInput {
    pub id: String,
    pub profile_id: String,
}

impl DeleteImageInput {
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

pub struct ForceDeleteImageInput {
    pub id: String,
    pub profile_id: String,
}

impl ForceDeleteImageInput {
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

pub struct GetUnusedImagesInput {
    pub profile_id: String,
    pub days_old: i32,
}

impl GetUnusedImagesInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        Ok(())
    }
}

pub struct DeleteUnusedImagesInput {
    pub profile_id: String,
    pub days_old: i32,
    pub confirm: bool,
}

impl DeleteUnusedImagesInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        if !self.confirm {
            return Err("Confirmation required to delete unused images".to_string());
        }
        Ok(())
    }
}

pub struct TrackImageUsageInput {
    pub profile_id: String,
    pub image_id: String,
    pub performance_id: String,
}

impl TrackImageUsageInput {
    pub fn validate(&self) -> Result<(), String> {
        if self.profile_id.trim().is_empty() {
            return Err("profile_id cannot be empty".to_string());
        }
        if self.image_id.trim().is_empty() {
            return Err("image_id cannot be empty".to_string());
        }
        if self.performance_id.trim().is_empty() {
            return Err("performance_id cannot be empty".to_string());
        }
        Ok(())
    }
}
