use global_hotkey::{
    hotkey::{Code, HotKey, Modifiers},
    GlobalHotKeyEvent, GlobalHotKeyManager,
};
use std::sync::mpsc::{self, Receiver, Sender};

/// Manages global hotkey registration and event handling
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
    receiver: Receiver<GlobalHotKeyEvent>,
}

impl HotkeyManager {
    /// Register a new global hotkey
    ///
    /// Default hotkey:
    /// - macOS: Cmd+Shift+P
    /// - Windows/Linux: Ctrl+Shift+P
    pub fn register() -> Result<Self, String> {
        // Create global hotkey manager
        let manager = GlobalHotKeyManager::new()
            .map_err(|e| format!("Failed to create hotkey manager: {}", e))?;

        // Platform-specific modifiers
        #[cfg(target_os = "macos")]
        let modifiers = Modifiers::SUPER | Modifiers::SHIFT;

        #[cfg(not(target_os = "macos"))]
        let modifiers = Modifiers::CONTROL | Modifiers::SHIFT;

        // Create hotkey (Cmd/Ctrl + Shift + P)
        let hotkey = HotKey::new(Some(modifiers), Code::KeyP);

        // Register the hotkey
        manager
            .register(hotkey)
            .map_err(|e| format!("Failed to register hotkey: {}. Is it already in use by another application?", e))?;

        // Set up event receiver
        let receiver = GlobalHotKeyEvent::receiver();

        println!("✓ Global hotkey registered successfully");
        #[cfg(target_os = "macos")]
        println!("  Press Cmd+Shift+P to trigger");
        #[cfg(not(target_os = "macos"))]
        println!("  Press Ctrl+Shift+P to trigger");

        Ok(HotkeyManager {
            manager,
            hotkey,
            receiver,
        })
    }

    /// Run the event loop (blocks on main thread - required for macOS)
    ///
    /// Accepts a channel sender to communicate hotkey events to async runtime
    pub fn run_event_loop(self, tx: Sender<()>) -> Result<(), String> {
        println!("Hotkey event loop running on main thread...");

        loop {
            // Check for hotkey events (non-blocking)
            if let Ok(event) = self.receiver.try_recv() {
                if event.id == self.hotkey.id() {
                    println!("Hotkey pressed!");

                    // Send signal to async runtime
                    if let Err(e) = tx.send(()) {
                        eprintln!("Failed to send hotkey event: {}", e);
                    }
                }
            }

            // Small sleep to prevent busy-waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }
}

impl Drop for HotkeyManager {
    fn drop(&mut self) {
        // Unregister hotkey on cleanup
        if let Err(e) = self.manager.unregister(self.hotkey) {
            eprintln!("Warning: Failed to unregister hotkey: {}", e);
        }
    }
}
