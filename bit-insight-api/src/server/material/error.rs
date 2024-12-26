use crate::error::{APIError, ErrorEntity};
use derive_more::Display;
use redis::RedisError;
use std::error::Error;

#[derive(Debug, Display)]
pub enum MaterialError {
    #[display("params error: {_0}")]
    Params(String),
    #[display("unauthorized")]
    Unauthorized,
    #[display("service error: {_0}")]
    Service(String),
    #[display("database error: {source}")]
    Database { source: sqlx::Error },
    #[display("redis error: {source}")]
    Redis { source: RedisError },
    #[display("material not found")]
    NotFound,
}

impl MaterialError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Params(_) => 10001,
            Self::Unauthorized => 10002,
            Self::Service(_) => 10003,
            Self::Database { .. } => 10004,
            Self::Redis { .. } => 10005,
            Self::NotFound => 10006,
        }
    }
}

impl Error for MaterialError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database { source: ref e } => Some(e),
            Self::Redis { source: ref e } => Some(e),
            _ => None,
        }
    }
}

impl From<sqlx::Error> for MaterialError {
    fn from(from: sqlx::Error) -> MaterialError {
        MaterialError::Database { source: from }
    }
}

impl From<RedisError> for MaterialError {
    fn from(from: RedisError) -> MaterialError {
        MaterialError::Redis { source: from }
    }
}

impl From<MaterialError> for APIError {
    fn from(from: MaterialError) -> APIError {
        let e = ErrorEntity {
            code: from.code(),
            message: from.to_string(),
        };
        match from {
            MaterialError::Params(_) => APIError::BadRequest(e),
            MaterialError::Unauthorized => APIError::Unauthorized(e),
            MaterialError::Service(_) | MaterialError::Database { .. } | MaterialError::Redis { .. } => {
                APIError::InternalError(e)
            }
            MaterialError::NotFound => APIError::NotFound(e),
        }
    }
}
