use arboard::Clipboard;
use std::error::Error;
use std::fmt;

const MAX_CLIPBOARD_SIZE: usize = 100_000; // 100KB limit

/// Custom error type for clipboard operations
#[derive(Debug)]
pub enum ClipboardError {
    /// Failed to access clipboard
    AccessError(String),
    /// Clipboard is empty
    Empty,
    /// Clipboard contains non-text content
    NonTextContent,
    /// Clipboard text exceeds size limit
    TooLarge(usize),
}

impl fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClipboardError::AccessError(msg) => write!(f, "Clipboard access error: {}", msg),
            ClipboardError::Empty => write!(f, "Clipboard is empty"),
            ClipboardError::NonTextContent => {
                write!(f, "Clipboard contains non-text content (images, files, etc.)")
            }
            ClipboardError::TooLarge(size) => write!(
                f,
                "Clipboard text too large ({} bytes, max {} bytes)",
                size, MAX_CLIPBOARD_SIZE
            ),
        }
    }
}

impl Error for ClipboardError {}

/// Clipboard handler for reading and writing text
pub struct ClipboardHandler {
    clipboard: Clipboard,
}

impl ClipboardHandler {
    /// Create a new clipboard handler
    pub fn new() -> Result<Self, ClipboardError> {
        Clipboard::new()
            .map(|clipboard| Self { clipboard })
            .map_err(|e| ClipboardError::AccessError(e.to_string()))
    }

    /// Get text from clipboard
    pub fn get_text(&mut self) -> Result<String, ClipboardError> {
        let text = self
            .clipboard
            .get_text()
            .map_err(|e| {
                // Check if error is due to empty clipboard
                let err_str = e.to_string();
                if err_str.contains("empty") || err_str.contains("unavailable") {
                    ClipboardError::Empty
                } else {
                    ClipboardError::AccessError(err_str)
                }
            })?;

        // Check if clipboard is effectively empty (whitespace only)
        if text.trim().is_empty() {
            return Err(ClipboardError::Empty);
        }

        // Check size limit
        if text.len() > MAX_CLIPBOARD_SIZE {
            return Err(ClipboardError::TooLarge(text.len()));
        }

        Ok(text)
    }

    /// Set text to clipboard
    pub fn set_text(&mut self, text: &str) -> Result<(), ClipboardError> {
        self.clipboard
            .set_text(text.to_string())
            .map_err(|e| ClipboardError::AccessError(e.to_string()))
    }
}

impl Default for ClipboardHandler {
    fn default() -> Self {
        Self::new().expect("Failed to initialize clipboard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_creation() {
        let result = ClipboardHandler::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_clipboard_round_trip() {
        let mut handler = ClipboardHandler::new().unwrap();

        // Set some text
        let test_text = "Hello, clipboard!";
        handler.set_text(test_text).unwrap();

        // Get it back
        let result = handler.get_text().unwrap();
        assert_eq!(result, test_text);
    }

    #[test]
    fn test_clipboard_trim() {
        let mut handler = ClipboardHandler::new().unwrap();

        // Set whitespace-only text
        handler.set_text("   \n\t   ").unwrap();

        // Should return Empty error
        let result = handler.get_text();
        assert!(matches!(result, Err(ClipboardError::Empty)));
    }

    #[test]
    fn test_max_clipboard_size() {
        // Create text larger than MAX_CLIPBOARD_SIZE
        let large_text = "x".repeat(MAX_CLIPBOARD_SIZE + 1);

        let mut handler = ClipboardHandler::new().unwrap();
        handler.set_text(&large_text).unwrap();

        let result = handler.get_text();
        assert!(matches!(result, Err(ClipboardError::TooLarge(_))));
    }
}
