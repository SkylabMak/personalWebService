use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role_id: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}

impl User {
    pub fn new(
        id: String,
        username: String,
        email: String,
        password_hash: String,
        role_id: String,
        created_at: String,
        updated_at: Option<String>,
    ) -> Self {
        Self {
            id,
            username,
            email,
            password_hash,
            role_id,
            created_at,
            updated_at,
        }
    }
}
