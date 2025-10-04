use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyManager,
};
use std::error::Error;
use std::fmt;

/// Custom error type for hotkey operations
#[derive(Debug)]
pub enum HotkeyError {
    /// Failed to create hotkey manager
    ManagerCreationFailed(String),
    /// Failed to register hotkey
    RegistrationFailed(String),
    /// Hotkey already in use
    AlreadyInUse,
}

impl fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HotkeyError::ManagerCreationFailed(msg) => {
                write!(f, "Failed to create hotkey manager: {}", msg)
            }
            HotkeyError::RegistrationFailed(msg) => {
                write!(f, "Failed to register hotkey: {}", msg)
            }
            HotkeyError::AlreadyInUse => {
                write!(f, "Hotkey is already in use by another application")
            }
        }
    }
}

impl Error for HotkeyError {}

/// Hotkey manager for registering and handling global hotkeys
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

impl HotkeyManager {
    /// Create a new hotkey manager with default hotkey (Cmd+Shift+P / Ctrl+Shift+P)
    pub fn new() -> Result<Self, HotkeyError> {
        // Create the global hotkey manager
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| HotkeyError::ManagerCreationFailed(e.to_string()))?;

        // Define the hotkey: Cmd+Shift+] on macOS, Ctrl+Shift+] on Windows/Linux
        #[cfg(target_os = "macos")]
        let modifiers = Modifiers::SUPER | Modifiers::SHIFT;

        #[cfg(not(target_os = "macos"))]
        let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

        let hotkey = HotKey::new(Some(modifiers), Code::BracketRight);

        Ok(Self {
            manager,
            hotkey,
        })
    }

    /// Register the hotkey
    pub fn register(&self) -> Result<(), HotkeyError> {
        self.manager
            .register(self.hotkey)
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("already") || err_str.contains("in use") {
                    HotkeyError::AlreadyInUse
                } else {
                    HotkeyError::RegistrationFailed(err_str)
                }
            })
    }

    /// Unregister the hotkey
    pub fn unregister(&self) -> Result<(), HotkeyError> {
        self.manager
            .unregister(self.hotkey)
            .map_err(|e| HotkeyError::RegistrationFailed(e.to_string()))
    }

    /// Get the hotkey description string
    pub fn hotkey_description(&self) -> String {
        #[cfg(target_os = "macos")]
        return "Cmd+Shift+]".to_string();

        #[cfg(not(target_os = "macos"))]
        return "Ctrl+Shift+]".to_string();
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        // Unregister hotkey on drop
        let _ = self.unregister();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hotkey_manager_creation() {
        let result = HotkeyManager::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_hotkey_description() {
        let manager = HotkeyManager::new().unwrap();
        let desc = manager.hotkey_description();

        #[cfg(target_os = "macos")]
        assert_eq!(desc, "Cmd+Shift+]");

        #[cfg(not(target_os = "macos"))]
        assert_eq!(desc, "Ctrl+Shift+]");
    }

}
