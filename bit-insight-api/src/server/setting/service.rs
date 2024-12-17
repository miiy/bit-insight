use super::dto::*;
use super::error::SettingError;
use super::model::Setting;
use sqlx::MySqlPool;

pub struct Service;

impl Service {
    pub async fn detail(key: &str, pool: &MySqlPool) -> Result<DetailResponse, SettingError> {
        let user_id = 0;
        let setting_option = Setting::find(&pool, user_id, key).await?;
        if let Some(setting) = setting_option {
            Ok(DetailResponse {
                value: setting.value,
            })
        } else {
            Err(SettingError::NotFound)
        }
    }

    pub async fn update(
        request: &UpdateRequest,
        pool: &MySqlPool,
    ) -> Result<UpdateResponse, SettingError> {
        let user_id = 0;
        let setting_option = Setting::find(&pool, user_id, &request.key).await?;
        if let Some(setting) = setting_option {
            let lii = Setting::update(&pool, user_id, &request.key, &request.value).await?;
            if lii > 0 {
                Ok(UpdateResponse {})
            } else {
                Err(SettingError::Database {
                    source: sqlx::Error::RowNotFound,
                })
            }
        } else {
            let ra = Setting::create(&pool, user_id, &request.key, &request.value).await?;
            if ra > 0 {
                Ok(UpdateResponse {})
            } else {
                Err(SettingError::Database {
                    source: sqlx::Error::RowNotFound,
                })
            }
        }
    }
}
