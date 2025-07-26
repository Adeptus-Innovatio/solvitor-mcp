use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, schemars::JsonSchema)]
pub struct DecodeRequest {
    pub program_id: String,
    pub url: Option<String>,
}
