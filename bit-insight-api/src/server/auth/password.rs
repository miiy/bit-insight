use super::error::AuthError;

pub type Cost = u32;
pub const DEFAULT_COST: Cost = 10;

pub fn bcrypt_hash(password: &str, cost: Cost) -> Result<String, AuthError> {
    Ok(bcrypt::hash(password, cost)?)
}

pub fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, AuthError> {
    Ok(bcrypt::verify(password, hash)?)
}
