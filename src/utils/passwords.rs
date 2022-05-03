use pwhash::bcrypt::{hash_with, verify, BcryptSetup};
use std::sync::Arc;

#[inline(always)]
pub fn verify_password(hashed_pass: String, input_pass: String) -> bool {
    verify(input_pass, hashed_pass.as_str())
}

pub fn hash_password(salt: Arc<&str>, input_pass: String) -> Result<String, ()> {
    match hash_with(
        BcryptSetup {
            salt: Some(*salt),
            ..Default::default()
        },
        input_pass,
    ) {
        Ok(res) => Ok(res),
        _ => Err(()),
    }
}
