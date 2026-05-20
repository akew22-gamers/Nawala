use crate::error::AppResult;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use rand_core::OsRng;

#[allow(dead_code)]
fn password_hasher() -> AppResult<Argon2<'static>> {
    let params = Params::new(65_536, 3, 4, None)
        .map_err(|e| crate::error::AppError::Auth(format!("Invalid Argon2 params: {e}")))?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

#[allow(dead_code)]
pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = password_hasher()?;
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| crate::error::AppError::Auth(format!("Failed to hash password: {e}")))?;
    Ok(hash.to_string())
}

#[allow(dead_code)]
pub fn verify_password(password: &str, hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| crate::error::AppError::Auth(format!("Invalid hash format: {e}")))?;
    let argon2 = password_hasher()?;
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
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
