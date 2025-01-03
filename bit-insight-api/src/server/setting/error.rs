use crate::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum SettingError {
    #[display("params error: {_0}")]
    Params(String),
    #[display("service error: {_0}")]
    Service(String),
    #[display("database error: {source}")]
    Database { source: sqlx::Error },
    #[display("redis error: {source}")]
    Redis { source: RedisError },
    #[display("setting not found")]
    NotFound,
    #[display("key not found: {_0}")]
    KeyNotFound(String),
}

impl SettingError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Params(_) => 10001,
            Self::Service(_) => 10002,
            Self::Database { .. } => 10003,
            Self::Redis { .. } => 10004,
            Self::NotFound => 10005,
            Self::KeyNotFound(_) => 10006,
        }
    }
}

impl Error for SettingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for SettingError {
    fn from(from: sqlx::Error) -> SettingError {
        SettingError::Database { source: from }
    }
}

impl From<RedisError> for SettingError {
    fn from(from: RedisError) -> SettingError {
        SettingError::Redis { source: from }
    }
}

impl From<serde_json::Error> for SettingError {
    fn from(from: serde_json::Error) -> SettingError {
        SettingError::Service(from.to_string())
    }
}

impl From<SettingError> for APIError {
    fn from(from: SettingError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            SettingError::Params(_) => APIError::BadRequest(e),
            SettingError::Service(_)
            | SettingError::Database { .. }
            | SettingError::Redis { .. } => APIError::InternalError(e),
            SettingError::NotFound => APIError::NotFound(e),
            SettingError::KeyNotFound(_) => APIError::NotFound(e),
        }
    }
}
