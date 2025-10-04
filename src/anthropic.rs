//! Anthropic Claude API client implementation
//!
//! IMPORTANT: This module uses ONLY reqwest and serde_json for API communication.
//! DO NOT introduce any Anthropic-specific crates (e.g., anthropic-sdk, claude-api, etc.)
//!
//! API Documentation: https://docs.claude.com/en/api/messages

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::time::Duration;

/// API endpoint for Claude Messages API
const API_ENDPOINT: &str = "https://api.anthropic.com/v1/messages";
const API_VERSION: &str = "2023-06-01";

/// Default model for testing (Claude 3.5 Haiku - fast and cost-effective)
pub const DEFAULT_MODEL: &str = "claude-3-5-haiku-20241022";

/// Default timeout for API requests (30 seconds)
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

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

/// Custom error type for API operations
#[derive(Debug)]
pub enum ApiError {
    /// Network or HTTP error
    NetworkError(String),
    /// API returned an error response
    ApiError { status: u16, message: String },
    /// Failed to parse response
    ParseError(String),
    /// Invalid prompt template
    TemplateError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            ApiError::ApiError { status, message } => {
                write!(f, "API error (status {}): {}", status, message)
            }
            ApiError::ParseError(msg) => write!(f, "Failed to parse response: {}", msg),
            ApiError::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl Error for ApiError {}

/// API client for communicating with Anthropic Claude API
pub struct ApiClient {
    api_key: String,
    client: reqwest::Client,
    model: String,
}

impl ApiClient {
    /// Create a new API client
    ///
    /// # Arguments
    /// * `api_key` - Anthropic API key
    /// * `model` - Model name (defaults to DEFAULT_MODEL if None)
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .expect("Failed to build HTTP client");

        Self {
            api_key,
            client,
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        }
    }

    /// Process text by sending it to Claude API with a prompt template
    ///
    /// # Arguments
    /// * `prompt_template` - Template containing {clipboard_text} placeholder
    /// * `clipboard_text` - Text to inject into the template
    ///
    /// # Returns
    /// The AI-processed text response
    pub async fn process_text(
        &self,
        prompt_template: &str,
        clipboard_text: &str,
    ) -> Result<String, ApiError> {
        // Format the prompt by replacing placeholder
        let formatted_prompt = self.format_prompt(prompt_template, clipboard_text)?;

        // Send to API
        self.send_message(&formatted_prompt).await
    }

    /// Format prompt by replacing {clipboard_text} placeholder
    fn format_prompt(
        &self,
        template: &str,
        clipboard_text: &str,
    ) -> Result<String, ApiError> {
        if !template.contains("{clipboard_text}") {
            return Err(ApiError::TemplateError(
                "Template must contain {clipboard_text} placeholder".to_string(),
            ));
        }

        Ok(template.replace("{clipboard_text}", clipboard_text))
    }

    /// Send a message to Claude API
    async fn send_message(&self, user_message: &str) -> Result<String, ApiError> {
        let request_body = ApiRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            }],
        };

        let response = self
            .client
            .post(API_ENDPOINT)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", API_VERSION)
            .header("content-type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(e.to_string()))?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ApiError::ApiError {
                status: status.as_u16(),
                message: error_text,
            });
        }

        let api_response: ApiResponse = response
            .json()
            .await
            .map_err(|e| ApiError::ParseError(e.to_string()))?;

        // Extract text from first content block
        if let Some(content) = api_response.content.first() {
            Ok(content.text.clone())
        } else {
            Err(ApiError::ParseError("No content in API response".to_string()))
        }
    }
}

/// Sends a prompt to Claude API and returns the response text
///
/// # Arguments
/// * `api_key` - Anthropic API key
/// * `user_message` - The message to send to Claude
/// * `model` - Model name (e.g., "claude-3-5-haiku-20241022" or use DEFAULT_MODEL)
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
            model: DEFAULT_MODEL.to_string(),
            max_tokens: 1024,
            messages: vec![Message {
                role: "user".to_string(),
                content: "Hello".to_string(),
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("claude-3-5-haiku"));
        assert!(json.contains("Hello"));
    }

    #[test]
    fn test_api_client_creation() {
        let client = ApiClient::new("sk-test-key".to_string(), None);
        assert_eq!(client.model, DEFAULT_MODEL);

        let client_custom = ApiClient::new("sk-test-key".to_string(), Some("custom-model".to_string()));
        assert_eq!(client_custom.model, "custom-model");
    }

    #[test]
    fn test_format_prompt() {
        let client = ApiClient::new("sk-test-key".to_string(), None);

        let template = "Please improve: {clipboard_text}";
        let clipboard = "Hello world";

        let result = client.format_prompt(template, clipboard).unwrap();
        assert_eq!(result, "Please improve: Hello world");
    }

    #[test]
    fn test_format_prompt_missing_placeholder() {
        let client = ApiClient::new("sk-test-key".to_string(), None);

        let template = "This template has no placeholder";
        let clipboard = "Hello world";

        let result = client.format_prompt(template, clipboard);
        assert!(result.is_err());

        if let Err(ApiError::TemplateError(msg)) = result {
            assert!(msg.contains("placeholder"));
        } else {
            panic!("Expected TemplateError");
        }
    }

    #[test]
    fn test_format_prompt_with_multiple_placeholders() {
        let client = ApiClient::new("sk-test-key".to_string(), None);

        let template = "Input: {clipboard_text}\nProcess: {clipboard_text}";
        let clipboard = "test";

        let result = client.format_prompt(template, clipboard).unwrap();
        assert_eq!(result, "Input: test\nProcess: test");
    }
}
