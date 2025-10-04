//! Anthropic Claude API client implementation
//!
//! IMPORTANT: This module uses ONLY reqwest and serde_json for API communication.
//! DO NOT introduce any Anthropic-specific crates (e.g., anthropic-sdk, claude-api, etc.)
//!
//! API Documentation: https://docs.claude.com/en/api/messages

use serde::{Deserialize, Serialize};
use std::error::Error;

/// API endpoint for Claude Messages API
const API_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct ContentBlock {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
    #[allow(dead_code)]
    id: String,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    role: String,
}

/// Sends a prompt to Claude API and returns the response text
///
/// # Arguments
/// * `api_key` - Anthropic API key
/// * `user_message` - The message to send to Claude
/// * `model` - Model name (e.g., "claude-sonnet-4-5")
/// * `max_tokens` - Maximum tokens in response
///
/// # Returns
/// The text response from Claude
pub async fn send_message(
    api_key: &str,
    user_message: &str,
    model: &str,
    max_tokens: u32,
) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let request_body = ApiRequest {
        model: model.to_string(),
        max_tokens,
        messages: vec![Message {
            role: "user".to_string(),
            content: user_message.to_string(),
        }],
    };

    let response = client
        .post(API_ENDPOINT)
        .header("x-api-key", api_key)
        .header("anthropic-version", API_VERSION)
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await?;
        return Err(format!("API error {}: {}", status, error_text).into());
    }

    let api_response: ApiResponse = response.json().await?;

    // Extract text from first content block
    if let Some(content) = api_response.content.first() {
        Ok(content.text.clone())
    } else {
        Err("No content in API response".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_request_structure() {
        let request = ApiRequest {
            model: "claude-sonnet-4-5".to_string(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("claude-sonnet-4-5"));
        assert!(json.contains("Hello"));
    }
}
