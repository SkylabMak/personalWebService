use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, Params,
};
use crate::application::errors::ApplicationError;

pub struct PasswordService {
    params: Params,
    pub salt: String,
}

impl PasswordService {
    pub fn new(salt: String, memory_cost: u32, iterations: u32, parallelism: u32) -> Self {
        let params = Params::new(memory_cost, iterations, parallelism, None)
            .expect("Invalid Argon2 parameters");
        Self { params, salt }
    }

    pub fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool, ApplicationError> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            self.params.clone(),
        );

        let parsed_hash = PasswordHash::new(password_hash)
            .map_err(|e| ApplicationError::Internal { message: format!("Invalid password hash: {}", e) })?;

        // Use the salt from the hash itself for verification
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    pub fn hash_password(&self, password: &str) -> Result<String, ApplicationError> {
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            self.params.clone(),
        );
        
        let salt = SaltString::from_b64(&self.salt)
            .map_err(|e| ApplicationError::Internal { message: format!("Invalid salt: {}", e) })?;

        let password_hash = argon2.hash_password(password.as_bytes(), &salt)
            .map_err(|e| ApplicationError::Internal { message: format!("Password hashing failed: {}", e) })?
            .to_string();

        Ok(password_hash)
    }
}
