use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub jti: String,      // JWT ID
    pub sub: String,      // user_id
    pub role: String,     // user role
    pub exp: usize,       // expiration
    pub iat: usize,       // issued at
}
