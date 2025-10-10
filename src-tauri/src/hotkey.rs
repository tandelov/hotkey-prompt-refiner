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
    /// Create a new hotkey manager with configurable hotkey
    pub fn new(modifiers_str: &str, key_str: &str) -> Result<Self, HotkeyError> {
        // Create the global hotkey manager
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| HotkeyError::ManagerCreationFailed(e.to_string()))?;

        // Parse modifiers from config
        let modifiers = Self::parse_modifiers(modifiers_str)?;

        // Parse key code from config
        let key_code = Self::parse_key_code(key_str)?;

        let hotkey = HotKey::new(Some(modifiers), key_code);

        Ok(Self {
            manager,
            hotkey,
        })
    }

    /// Parse modifier keys from string (e.g., "cmd+shift", "ctrl+alt")
    fn parse_modifiers(modifiers_str: &str) -> Result<Modifiers, HotkeyError> {
        let mut modifiers = Modifiers::empty();

        for part in modifiers_str.to_lowercase().split('+') {
            match part.trim() {
                "cmd" | "super" => modifiers |= Modifiers::SUPER,
                "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
                "alt" | "option" => modifiers |= Modifiers::ALT,
                "shift" => modifiers |= Modifiers::SHIFT,
                "" => {}, // ignore empty parts
                unknown => {
                    return Err(HotkeyError::RegistrationFailed(
                        format!("Unknown modifier: {}", unknown)
                    ));
                }
            }
        }

        Ok(modifiers)
    }

    /// Parse key code from string (e.g., "BracketRight", "KeyP")
    fn parse_key_code(key_str: &str) -> Result<Code, HotkeyError> {
        match key_str {
            "BracketRight" => Ok(Code::BracketRight),
            "BracketLeft" => Ok(Code::BracketLeft),
            "KeyP" => Ok(Code::KeyP),
            "KeyK" => Ok(Code::KeyK),
            "Semicolon" => Ok(Code::Semicolon),
            "Space" => Ok(Code::Space),
            "Enter" => Ok(Code::Enter),
            "Backslash" => Ok(Code::Backslash),
            _ => Err(HotkeyError::RegistrationFailed(
                format!("Unknown key code: {}. See global-hotkey docs for valid codes.", key_str)
            )),
        }
    }

    /// Get the hotkey description string based on configured modifiers and key
    pub fn hotkey_description(&self, modifiers_str: &str, key_str: &str) -> String {
        // Convert modifiers string to display format
        let mods = modifiers_str
            .split('+')
            .map(|m| {
                match m.trim().to_lowercase().as_str() {
                    "cmd" | "super" => "Cmd",
                    "ctrl" | "control" => "Ctrl",
                    "alt" | "option" => "Alt",
                    "shift" => "Shift",
                    _ => m.trim(),
                }
            })
            .collect::<Vec<_>>()
            .join("+");

        // Convert key code to display format
        let key = match key_str {
            "BracketRight" => "]",
            "BracketLeft" => "[",
            "Backslash" => "\\",
            "Semicolon" => ";",
            "Space" => "Space",
            "Enter" => "Enter",
            key if key.starts_with("Key") => &key[3..], // Strip "Key" prefix
            key => key,
        };

        format!("{}+{}", mods, key)
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
        let result = HotkeyManager::new("cmd+shift", "BracketRight");
        assert!(result.is_ok());
    }

    #[test]
    fn test_hotkey_description() {
        let manager = HotkeyManager::new("cmd+shift", "BracketRight").unwrap();
        let desc = manager.hotkey_description("cmd+shift", "BracketRight");
        assert_eq!(desc, "Cmd+Shift+]");

        let manager2 = HotkeyManager::new("ctrl+alt", "KeyP").unwrap();
        let desc2 = manager2.hotkey_description("ctrl+alt", "KeyP");
        assert_eq!(desc2, "Ctrl+Alt+P");
    }

    #[test]
    fn test_parse_modifiers() {
        let mods = HotkeyManager::parse_modifiers("cmd+shift").unwrap();
        assert!(mods.contains(Modifiers::SUPER));
        assert!(mods.contains(Modifiers::SHIFT));

        let mods2 = HotkeyManager::parse_modifiers("ctrl+alt").unwrap();
        assert!(mods2.contains(Modifiers::CONTROL));
        assert!(mods2.contains(Modifiers::ALT));
    }

    #[test]
    fn test_parse_key_code() {
        assert!(HotkeyManager::parse_key_code("BracketRight").is_ok());
        assert!(HotkeyManager::parse_key_code("KeyP").is_ok());
        assert!(HotkeyManager::parse_key_code("InvalidKey").is_err());
    }

}
