use super::error::SettingError;

pub fn validate_key(key: &str) -> Result<(), SettingError> {
    if key.is_empty() {
        return Err(SettingError::KeyNotFound("".into()));
    }
    Ok(())
}
