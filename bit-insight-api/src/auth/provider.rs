use serde::Serialize;
use std::future::Future;

// authenticated user
#[derive(Debug, Serialize, Clone)]
pub struct AuthenticatedUser {
    pub id: u64,
    pub username: String,
}

// provider
pub trait AuthenticationProvider {
    fn get_user(&self, sub: String) -> impl Future<Output = Result<AuthenticatedUser, ProviderError>> + Send;
    // fn get_user(&self, sub: String) -> Result<AuthenticatedUser, ProviderError>;
}

// error
pub enum ProviderError {
    UserNotFound,
    DatabaseError { source: sqlx::Error },
    RedisError { source: redis::RedisError },
}

impl From<sqlx::Error> for ProviderError {
    fn from(e: sqlx::Error) -> Self {
        ProviderError::DatabaseError { source: e }
    }
}

impl From<redis::RedisError> for ProviderError {
    fn from(e: redis::RedisError) -> Self {
        ProviderError::RedisError { source: e }
    }
}
