use crate::clipboard::{ClipboardError, ClipboardHandler};
use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use std::error::Error;
use std::fmt;
use std::thread;
use std::time::Duration;

/// Custom error type for paste operations
#[derive(Debug)]
pub enum PasteError {
    /// Failed to set clipboard
    ClipboardError(ClipboardError),
    /// Failed to simulate keyboard input
    SimulationError(String),
    /// Accessibility permissions missing (macOS)
    PermissionsError,
}

impl fmt::Display for PasteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PasteError::ClipboardError(e) => write!(f, "Clipboard error: {}", e),
            PasteError::SimulationError(msg) => write!(f, "Paste simulation error: {}", msg),
            PasteError::PermissionsError => write!(
                f,
                "Accessibility permissions required. Please enable in System Settings."
            ),
        }
    }
}

impl Error for PasteError {}

impl From<ClipboardError> for PasteError {
    fn from(e: ClipboardError) -> Self {
        PasteError::ClipboardError(e)
    }
}

/// Paste handler for simulating keyboard input
pub struct PasteHandler {
    clipboard: ClipboardHandler,
}

impl PasteHandler {
    /// Create a new paste handler
    pub fn new() -> Result<Self, PasteError> {
        Ok(Self {
            clipboard: ClipboardHandler::new()?,
        })
    }

    /// Paste text at the current cursor location
    ///
    /// This function:
    /// 1. Puts the text in the clipboard
    /// 2. Simulates Cmd+V (macOS) or Ctrl+V (Windows/Linux)
    /// 3. Returns error if paste fails
    ///
    /// On macOS, this requires Accessibility permissions.
    pub fn paste_text(&mut self, text: &str) -> Result<(), PasteError> {
        // Step 1: Put text in clipboard
        self.clipboard.set_text(text)?;

        // Step 2: Wait a moment for clipboard to be ready
        thread::sleep(Duration::from_millis(10));

        // Step 3: Simulate paste keystroke
        self.simulate_paste()?;

        Ok(())
    }

    /// Paste text with fallback to clipboard-only
    ///
    /// If paste simulation fails, the text will still be in the clipboard
    /// and the user can manually paste with Cmd+V/Ctrl+V.
    pub fn paste_text_with_fallback(&mut self, text: &str) -> Result<bool, PasteError> {
        // Always put text in clipboard first
        self.clipboard.set_text(text)?;

        // Try to simulate paste
        match self.simulate_paste() {
            Ok(()) => Ok(true), // Paste succeeded
            Err(e) => {
                // Paste failed, but text is in clipboard
                eprintln!("⚠ Auto-paste failed: {}", e);
                eprintln!("  Text copied to clipboard. Press Cmd+V/Ctrl+V to paste.");
                Ok(false) // Paste failed but clipboard has text
            }
        }
    }

    /// Simulate paste keystroke (Cmd+V on macOS, Ctrl+V on Windows/Linux)
    fn simulate_paste(&self) -> Result<(), PasteError> {
        let settings = Settings::default();
        let mut enigo = Enigo::new(&settings)
            .map_err(|e| PasteError::SimulationError(format!("Failed to create Enigo: {:?}", e)))?;

        #[cfg(target_os = "macos")]
        {
            // macOS: Cmd+V
            enigo.key(Key::Meta, Direction::Press)
                .map_err(|e| PasteError::SimulationError(format!("Failed to press Meta: {:?}", e)))?;
            thread::sleep(Duration::from_millis(5));
            enigo.key(Key::Unicode('v'), Direction::Click)
                .map_err(|e| PasteError::SimulationError(format!("Failed to press V: {:?}", e)))?;
            thread::sleep(Duration::from_millis(5));
            enigo.key(Key::Meta, Direction::Release)
                .map_err(|e| PasteError::SimulationError(format!("Failed to release Meta: {:?}", e)))?;
        }

        #[cfg(not(target_os = "macos"))]
        {
            // Windows/Linux: Ctrl+V
            enigo.key(Key::Control, Direction::Press)
                .map_err(|e| PasteError::SimulationError(format!("Failed to press Control: {:?}", e)))?;
            thread::sleep(Duration::from_millis(5));
            enigo.key(Key::Unicode('v'), Direction::Click)
                .map_err(|e| PasteError::SimulationError(format!("Failed to press V: {:?}", e)))?;
            thread::sleep(Duration::from_millis(5));
            enigo.key(Key::Control, Direction::Release)
                .map_err(|e| PasteError::SimulationError(format!("Failed to release Control: {:?}", e)))?;
        }

        Ok(())
    }

    /// Check if Accessibility permissions are granted (macOS only)
    ///
    /// Note: This is a best-effort check. The most reliable way to know
    /// is to try pasting and see if it works.
    #[cfg(target_os = "macos")]
    pub fn check_permissions() -> bool {
        // On macOS, we can't easily check permissions without trying
        // The user will get a system prompt the first time they try to paste
        // For now, we'll assume permissions might not be granted
        // and handle errors gracefully
        true // Optimistic assumption
    }

    #[cfg(not(target_os = "macos"))]
    pub fn check_permissions() -> bool {
        true // No special permissions needed on Windows/Linux
    }
}

impl Default for PasteHandler {
    fn default() -> Self {
        Self::new().expect("Failed to initialize paste handler")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paste_handler_creation() {
        let result = PasteHandler::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_permissions_check() {
        // This should always return true (we can't reliably check on macOS)
        assert!(PasteHandler::check_permissions());
    }

    #[test]
    fn test_clipboard_integration() {
        let mut handler = PasteHandler::new().unwrap();

        // Test that we can at least set clipboard
        // (We can't test actual pasting without user interaction)
        let test_text = "Test paste content";

        // This should at least work (putting text in clipboard)
        let result = handler.clipboard.set_text(test_text);
        assert!(result.is_ok());

        // Verify it's in clipboard
        let clipboard_content = handler.clipboard.get_text().unwrap();
        assert_eq!(clipboard_content, test_text);
    }
}
