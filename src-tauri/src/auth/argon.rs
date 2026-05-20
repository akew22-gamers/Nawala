use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand_core::OsRng;
use crate::error::AppResult;

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| crate::error::AppError::Auth(format!("Failed to hash password: {}", e)))?;
    Ok(hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| crate::error::AppError::Auth(format!("Invalid hash format: {}", e)))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifies_matching_password_and_rejects_other_password() {
        // Hash the password "123456"
        let hash = hash_password("123456").expect("Failed to hash password");
        
        // Should return true for matching password
        let result_match = verify_password("123456", &hash).expect("Failed to verify password");
        assert!(result_match, "Expected true for matching password");
        
        // Should return false for different password
        let result_mismatch = verify_password("000000", &hash).expect("Failed to verify password");
        assert!(!result_mismatch, "Expected false for non-matching password");
    }
}
