use serde::Serialize;
use serde_json::Value;

// setting

#[derive(Debug, Serialize)]
pub struct DetailResponse {
    pub value: Value,
}

#[derive(Debug, Serialize)]
pub struct UpdateResponse {}
