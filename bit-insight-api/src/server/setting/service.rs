use super::dto::*;
use super::error::SettingError;
use super::model::{Setting, SettingValuePush};
use serde_json::Value;
use sqlx::MySqlPool;

pub struct Service;

impl Service {
    const SETTING_PUSH_KEY: &str = "push";

    pub async fn detail(
        user_id: u64,
        key: &str,
        pool: &MySqlPool,
    ) -> Result<DetailResponse, SettingError> {
        let setting_option = Setting::find(&pool, user_id, key).await?;

        if let Some(setting) = setting_option {
            Ok(DetailResponse {
                value: setting.value,
            })
        } else {
            // default value
            let value = match key {
                Self::SETTING_PUSH_KEY => serde_json::to_value(SettingValuePush::default())?,
                _ => {
                    return Err(SettingError::KeyNotFound(key.to_string()));
                }
            };
            Ok(DetailResponse { value: value })
        }
    }

    pub async fn update(
        user_id: u64,
        key: &str,
        value: Value,
        pool: &MySqlPool,
    ) -> Result<UpdateResponse, SettingError> {
        // Convert value to string based on key type
        let json_value = match key {
            // validate SettingPush
            Self::SETTING_PUSH_KEY => {
                let setting_push: SettingValuePush = serde_json::from_value(value)?;
                // Additional validation could go here if needed
                if setting_push.push_token.is_empty() {
                    return Err(SettingError::Params("push_token cannot be empty".into()));
                }
                serde_json::to_value(&setting_push)?
            }
            _ => {
                return Err(SettingError::KeyNotFound(key.to_string()));
            }
        };

        let setting_option = Setting::find(&pool, user_id, key).await?;

        // update or create
        if setting_option.is_some() {
            println!("update: {}", json_value);
            let last_insert_id = Setting::update(&pool, user_id, key, json_value).await?;
            if last_insert_id > 0 {
                Ok(UpdateResponse {})
            } else {
                Err(SettingError::Service("update setting failed".into()))
            }
        } else {
            println!("create: {}", json_value);
            let rows_affected = Setting::create(&pool, user_id, key, json_value).await?;
            if rows_affected > 0 {
                Ok(UpdateResponse {})
            } else {
                Err(SettingError::Service("create setting failed".into()))
            }
        }
    }
}
