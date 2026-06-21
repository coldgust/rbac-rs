use argon2::password_hash::SaltString;
use argon2::password_hash::errors;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
pub struct PasswordUtil;

impl PasswordUtil {
    pub fn hash(pass: &str) -> errors::Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        Ok(argon2.hash_password(pass.as_bytes(), &salt)?.to_string())
    }

    pub fn verify(pass: &str, hash: &str) -> errors::Result<()> {
        let argon = Argon2::default();
        let parsed_hash = PasswordHash::new(hash)?;
        argon.verify_password(pass.as_bytes(), &parsed_hash)
    }
}
