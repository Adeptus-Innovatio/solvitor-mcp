use std::sync::Arc;

use rmcp::{model::*, tool, Error as McpError, ServerHandler};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::solvitor_mcp::requests::*;

const SOLVITOR_API_BASE_URL: &str = "https://shaky-kelcy-wowinter13-940ce3be.koyeb.app/public-api";

#[derive(Clone)]
pub struct SolvitorApi {
    api_key: Arc<Mutex<String>>,
    client: reqwest::Client,
}

#[tool(tool_box)]
impl SolvitorApi {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key: Arc::new(Mutex::new(api_key)),
            client: reqwest::Client::new(),
        }
    }

    async fn make_request(&self, endpoint: &str, params: Option<Value>) -> Result<Value, McpError> {
        let url = format!("{}{}", SOLVITOR_API_BASE_URL, endpoint);
        let api_key = self.api_key.lock().await.clone();

        let request = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json");

        let request = if let Some(params) = params {
            request.json(&params)
        } else {
            request
        };

        let response = request
            .send()
            .await
            .map_err(|e| McpError::internal_error(format!("HTTP request error: {}", e), None))?;

        if response.status() != reqwest::StatusCode::OK {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            return Err(McpError::internal_error(
                format!("HTTP {}", error_text),
                None,
            ));
        }

        response
            .json::<Value>()
            .await
            .map_err(|e| McpError::internal_error(format!("JSON parsing error: {}", e), None))
    }


    // Extract IDL (Interface Definition Language) from any Solana program using reverse engineering techniques.
    #[tool(description = "Decode a program")]
    async fn decode(
        &self,
        #[tool(aggr)] request: DecodeRequest,
    ) -> Result<CallToolResult, McpError> {

        let url = request.url.unwrap_or("https://api.mainnet-beta.solana.com".to_string());
        let params = json!({
            "program_id": request.program_id,
            "url": url,
        });

        let response = self.make_request("/v1/decode", Some(params)).await?;

        let content = Content::json(response).map_err(|e| {
            McpError::internal_error(
                "Failed to serialize JSON response",
                Some(json!({"error": e.message})),
            )
        })?;

        Ok(CallToolResult::success(vec![content]))
    }
}

#[tool(tool_box)]
impl ServerHandler for SolvitorApi {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("This server provides tools to access the Solvitor API. Solvitor is an AI-powered platform that helps developers extract IDL files from closed-source Solana smart contracts and decompile them.".to_string()),
        }
    }
}