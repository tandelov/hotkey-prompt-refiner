use crate::anthropic::{ApiClient, ApiError};
use crate::clipboard::{ClipboardError, ClipboardHandler};
use crate::config::Config;
use crate::paste::{PasteError, PasteHandler};
use std::error::Error;
use std::fmt;

/// Custom error type for workflow operations
#[derive(Debug)]
pub enum WorkflowError {
    /// Error reading from clipboard
    ClipboardError(ClipboardError),
    /// Error calling API
    ApiError(ApiError),
    /// Error pasting response
    PasteError(PasteError),
}

impl fmt::Display for WorkflowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkflowError::ClipboardError(e) => write!(f, "Clipboard error: {}", e),
            WorkflowError::ApiError(e) => write!(f, "API error: {}", e),
            WorkflowError::PasteError(e) => write!(f, "Paste error: {}", e),
        }
    }
}

impl Error for WorkflowError {}

impl From<ClipboardError> for WorkflowError {
    fn from(e: ClipboardError) -> Self {
        WorkflowError::ClipboardError(e)
    }
}

impl From<ApiError> for WorkflowError {
    fn from(e: ApiError) -> Self {
        WorkflowError::ApiError(e)
    }
}

impl From<PasteError> for WorkflowError {
    fn from(e: PasteError) -> Self {
        WorkflowError::PasteError(e)
    }
}

/// Execute the complete workflow:
/// 1. Read clipboard text
/// 2. Process with Claude API
/// 3. Paste response at cursor
///
/// # Arguments
/// * `config` - Application configuration
/// * `api_client` - API client for Claude
///
/// # Returns
/// Ok(()) if workflow completed successfully, Err otherwise
pub async fn execute_workflow(
    config: &Config,
    api_client: &ApiClient,
) -> Result<(), WorkflowError> {
    println!("\n🚀 Starting workflow...");

    // Step 1: Read clipboard
    println!("📋 Step 1: Reading clipboard...");
    let mut clipboard = ClipboardHandler::new()?;
    let clipboard_text = clipboard.get_text()?;
    println!("   ✓ Clipboard text: \"{}\"", clipboard_text.lines().next().unwrap_or(&clipboard_text));

    // Step 2: Process with API
    println!("🤖 Step 2: Processing with Claude API...");
    let response = api_client.process_text(&config.prompt_template, &clipboard_text).await?;
    println!("   ✓ Received response ({} chars)", response.len());

    // Step 3: Paste response
    println!("📝 Step 3: Pasting response...");
    let mut paste_handler = PasteHandler::new()?;

    match paste_handler.paste_text_with_fallback(&response) {
        Ok(true) => {
            println!("   ✓ Response pasted successfully!");
        }
        Ok(false) => {
            println!("   ⚠ Auto-paste failed, but response is in clipboard");
            println!("   → Press Cmd+V (or Ctrl+V) to paste manually");
        }
        Err(e) => {
            println!("   ✗ Paste failed: {}", e);
            return Err(e.into());
        }
    }

    println!("✅ Workflow completed successfully!\n");
    Ok(())
}

/// Execute workflow with detailed error logging
pub async fn execute_workflow_with_logging(
    config: &Config,
    api_client: &ApiClient,
) -> Result<(), WorkflowError> {
    match execute_workflow(config, api_client).await {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("\n❌ Workflow failed:");
            match &e {
                WorkflowError::ClipboardError(ClipboardError::Empty) => {
                    eprintln!("   Clipboard is empty. Please copy some text first.");
                }
                WorkflowError::ClipboardError(ce) => {
                    eprintln!("   Clipboard error: {}", ce);
                }
                WorkflowError::ApiError(ae) => {
                    eprintln!("   API error: {}", ae);
                    eprintln!("   Check your API key and network connection.");
                }
                WorkflowError::PasteError(pe) => {
                    eprintln!("   Paste error: {}", pe);
                }
            }
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_error_display() {
        let err = WorkflowError::ClipboardError(ClipboardError::Empty);
        assert!(err.to_string().contains("Clipboard"));
    }
}
