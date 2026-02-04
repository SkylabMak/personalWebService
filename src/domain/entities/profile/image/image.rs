pub struct Image {
    pub id: String,
    pub profile_id: String,
    pub filename: String,
    pub original_filename: String,
    pub storage_url: String,
    pub file_size: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub mime_type: String,
    pub alt_text: Option<String>,
    pub caption: Option<String>,
    pub created_at: String,
}
