pub struct ImageUsage {
    pub id: String,
    pub image_id: String,
    pub performance_id: String,
    pub usage_count: i32,
    pub first_used_at: String,
    pub last_used_at: String,
}

pub struct ImageUsageInfo {
    pub performance_id: String,
    pub title: String,
    pub usage_count: i32,
    pub first_used_at: String,
    pub last_used_at: String,
}
