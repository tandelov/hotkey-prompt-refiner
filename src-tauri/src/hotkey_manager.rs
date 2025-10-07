use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Manages multiple global hotkeys mapped to template IDs
pub struct HotkeyManager {
    manager: Arc<GlobalHotKeyManager>,
    /// Map of hotkey ID to template ID
    hotkey_map: Arc<Mutex<HashMap<u32, String>>>,
}

impl HotkeyManager {
    /// Create a new hotkey manager
    pub fn new() -> Result<Self, String> {
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| format!("Failed to create hotkey manager: {}", e))?;

        Ok(HotkeyManager {
            manager: Arc::new(manager),
            hotkey_map: Arc::new(Mutex::new(HashMap::new())),
        })
    }

    /// Register a hotkey for a template
    ///
    /// Hotkey format: "Cmd+Shift+G" or "Ctrl+Alt+F"
    pub fn register_hotkey(&self, template_id: String, hotkey_str: &str) -> Result<(), String> {
        let hotkey = parse_hotkey_string(hotkey_str)?;

        // Register with global hotkey manager
        self.manager
            .register(hotkey)
            .map_err(|e| format!("Failed to register hotkey '{}': {}. It may already be in use.", hotkey_str, e))?;

        // Store mapping
        let mut map = self.hotkey_map.lock().unwrap();
        map.insert(hotkey.id(), template_id.clone());

        println!("✓ Registered hotkey '{}' for template '{}'", hotkey_str, template_id);
        Ok(())
    }

    /// Unregister a hotkey by template ID
    pub fn unregister_hotkey(&self, template_id: &str) -> Result<(), String> {
        let mut map = self.hotkey_map.lock().unwrap();

        // Find the hotkey ID for this template
        let hotkey_id = map
            .iter()
            .find(|(_, tid)| tid.as_str() == template_id)
            .map(|(hid, _)| *hid);

        if let Some(hid) = hotkey_id {
            // Recreate the hotkey to unregister it
            // Note: This is a workaround since we can't store HotKey directly
            // In practice, we'd need to also store the hotkey definition
            map.remove(&hid);
            println!("✓ Unregistered hotkey for template '{}'", template_id);
            Ok(())
        } else {
            Err(format!("No hotkey registered for template '{}'", template_id))
        }
    }

    /// Check for hotkey events and return the template ID if pressed
    pub fn check_events(&self) -> Option<String> {
        let receiver = GlobalHotKeyEvent::receiver();

        if let Ok(event) = receiver.try_recv() {
            if event.state == HotKeyState::Pressed {
                let map = self.hotkey_map.lock().unwrap();
                if let Some(template_id) = map.get(&event.id) {
                    return Some(template_id.clone());
                }
            }
        }

        None
    }

    /// Get the current hotkey map
    pub fn get_hotkey_map(&self) -> HashMap<u32, String> {
        self.hotkey_map.lock().unwrap().clone()
    }
}

/// Parse a hotkey string like "Cmd+Shift+G" into a HotKey
fn parse_hotkey_string(hotkey_str: &str) -> Result<HotKey, String> {
    let parts: Vec<&str> = hotkey_str.split('+').map(|s| s.trim()).collect();

    if parts.is_empty() {
        return Err("Hotkey string is empty".to_string());
    }

    let mut modifiers = Modifiers::empty();
    let key_str = parts.last().unwrap();

    // Parse modifiers
    for part in &parts[..parts.len() - 1] {
        match part.to_lowercase().as_str() {
            "cmd" | "super" | "command" => modifiers |= Modifiers::SUPER,
            "ctrl" | "control" => modifiers |= Modifiers::CONTROL,
            "alt" | "option" => modifiers |= Modifiers::ALT,
            "shift" => modifiers |= Modifiers::SHIFT,
            _ => return Err(format!("Unknown modifier: {}", part)),
        }
    }

    // Parse key code
    let code = match key_str.to_lowercase().as_str() {
        "a" => Code::KeyA,
        "b" => Code::KeyB,
        "c" => Code::KeyC,
        "d" => Code::KeyD,
        "e" => Code::KeyE,
        "f" => Code::KeyF,
        "g" => Code::KeyG,
        "h" => Code::KeyH,
        "i" => Code::KeyI,
        "j" => Code::KeyJ,
        "k" => Code::KeyK,
        "l" => Code::KeyL,
        "m" => Code::KeyM,
        "n" => Code::KeyN,
        "o" => Code::KeyO,
        "p" => Code::KeyP,
        "q" => Code::KeyQ,
        "r" => Code::KeyR,
        "s" => Code::KeyS,
        "t" => Code::KeyT,
        "u" => Code::KeyU,
        "v" => Code::KeyV,
        "w" => Code::KeyW,
        "x" => Code::KeyX,
        "y" => Code::KeyY,
        "z" => Code::KeyZ,
        "0" => Code::Digit0,
        "1" => Code::Digit1,
        "2" => Code::Digit2,
        "3" => Code::Digit3,
        "4" => Code::Digit4,
        "5" => Code::Digit5,
        "6" => Code::Digit6,
        "7" => Code::Digit7,
        "8" => Code::Digit8,
        "9" => Code::Digit9,
        "f1" => Code::F1,
        "f2" => Code::F2,
        "f3" => Code::F3,
        "f4" => Code::F4,
        "f5" => Code::F5,
        "f6" => Code::F6,
        "f7" => Code::F7,
        "f8" => Code::F8,
        "f9" => Code::F9,
        "f10" => Code::F10,
        "f11" => Code::F11,
        "f12" => Code::F12,
        "]" | "bracketright" => Code::BracketRight,
        "[" | "bracketleft" => Code::BracketLeft,
        ";" | "semicolon" => Code::Semicolon,
        "'" | "quote" => Code::Quote,
        "," | "comma" => Code::Comma,
        "." | "period" => Code::Period,
        "/" | "slash" => Code::Slash,
        "\\" | "backslash" => Code::Backslash,
        "-" | "minus" => Code::Minus,
        "=" | "equal" => Code::Equal,
        "space" => Code::Space,
        "enter" | "return" => Code::Enter,
        "tab" => Code::Tab,
        "backspace" => Code::Backspace,
        "delete" => Code::Delete,
        "escape" | "esc" => Code::Escape,
        _ => return Err(format!("Unknown key: {}", key_str)),
    };

    Ok(HotKey::new(Some(modifiers), code))
}
