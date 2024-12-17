use serde::{Deserialize, Serialize};

// setting
#[derive(Debug, Deserialize)]
pub struct DetailRequest {
    pub key: String,
}

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {}
