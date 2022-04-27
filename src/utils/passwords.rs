use pwhash::bcrypt::{hash_with, verify, BcryptSetup};

#[inline(always)]
pub fn verify_password(hashed_pass: String, input_pass: String) -> bool {
    verify(input_pass, hashed_pass.as_str())
}

pub fn hash_password(salt: String, input_pass: String) -> Result<String, ()> {
    match hash_with(
        BcryptSetup {
            salt: Some(salt.as_str()),
            ..Default::default()
        },
        input_pass,
    ) {
        Ok(res) => Ok(res),
        _ => Err(()),
    }
}
