// Backend modules from original hotkey-prompt-refiner
mod anthropic;
mod clipboard;
// mod config;  // Legacy CLI config - not used in Tauri
// mod hotkey;  // Legacy CLI hotkey - replaced by hotkey_manager
mod paste;
// mod workflow;  // Legacy CLI workflow - not used in Tauri

// Tauri configuration system
mod commands;
mod models;
mod hotkey_manager;
mod migration;

// History storage
mod lib {
    pub mod history;
}

use hotkey_manager::HotkeyManager;
use anthropic::ApiClient;
use lib::history::HistoryStore;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use once_cell::sync::Lazy;
use tauri::{Manager, AppHandle};
use tauri::tray::{TrayIconBuilder, MouseButton, MouseButtonState};

// Global history store
static HISTORY: Lazy<HistoryStore> = Lazy::new(|| HistoryStore::new());

// Global state for hotkey manager and API client
struct AppState {
    hotkey_manager: Arc<HotkeyManager>,
    api_client: Arc<Mutex<Option<ApiClient>>>,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// History commands
#[tauri::command]
fn get_history() -> Result<Vec<models::HistoryEntry>, String> {
    Ok(HISTORY.get_all())
}

#[tauri::command]
fn get_history_page(offset: usize, limit: usize) -> Result<Vec<models::HistoryEntry>, String> {
    Ok(HISTORY.get_page(offset, limit))
}

#[tauri::command]
fn clear_history() -> Result<(), String> {
    HISTORY.clear();
    Ok(())
}

#[tauri::command]
fn search_history(query: String) -> Result<Vec<models::HistoryEntry>, String> {
    Ok(HISTORY.search(&query))
}

#[tauri::command]
fn get_history_count() -> Result<usize, String> {
    Ok(HISTORY.count())
}

// Window management commands
#[tauri::command]
fn show_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn toggle_main_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

// Auto-launch commands
#[tauri::command]
fn is_autostart_enabled(app: AppHandle) -> Result<bool, String> {
    use tauri_plugin_autostart::ManagerExt;
    let autostart_manager = app.autolaunch();
    autostart_manager
        .is_enabled()
        .map_err(|e| format!("Failed to check autostart status: {}", e))
}

#[tauri::command]
fn enable_autostart(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let autostart_manager = app.autolaunch();
    autostart_manager
        .enable()
        .map_err(|e| format!("Failed to enable autostart: {}", e))
}

#[tauri::command]
fn disable_autostart(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_autostart::ManagerExt;
    let autostart_manager = app.autolaunch();
    autostart_manager
        .disable()
        .map_err(|e| format!("Failed to disable autostart: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load environment variables from .env file if present
    dotenv::dotenv().ok();

    println!("Hotkey Prompt Refiner v{}", env!("CARGO_PKG_VERSION"));
    println!("Starting Tauri GUI...\n");

    // Create hotkey manager
    let hotkey_manager = match HotkeyManager::new() {
        Ok(manager) => {
            println!("✓ Hotkey manager initialized");
            Arc::new(manager)
        }
        Err(e) => {
            eprintln!("✗ Failed to create hotkey manager: {}", e);
            std::process::exit(1);
        }
    };

    // Try to load API key and create client
    let api_client = Arc::new(Mutex::new(load_api_client()));

    // Load templates and register hotkeys
    if let Err(e) = register_template_hotkeys(&hotkey_manager) {
        eprintln!("Warning: Failed to register template hotkeys: {}", e);
    }

    let hotkey_manager_clone = Arc::clone(&hotkey_manager);
    let api_client_clone = Arc::clone(&api_client);

    // Spawn background thread to check for hotkey events
    std::thread::spawn(move || {
        println!("✓ Hotkey event loop started");
        loop {
            // Check for hotkey events
            if let Some(template_id) = hotkey_manager_clone.check_events() {
                println!("\n⌨️  Hotkey pressed for template: {}", template_id);

                // Get the API client
                let client_guard = api_client_clone.lock().unwrap();
                if let Some(ref api_client) = *client_guard {
                    // Execute the template workflow
                    let api_client = Arc::new(api_client.clone());
                    let template_id_clone = template_id.clone();

                    // Spawn async task to execute workflow
                    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
                    rt.block_on(async {
                        match execute_template_workflow(&template_id_clone, &api_client).await {
                            Ok(()) => {
                                println!("✅ Workflow completed successfully for template '{}'!\n", template_id_clone);
                            }
                            Err(e) => {
                                eprintln!("❌ Workflow failed for template '{}': {}\n", template_id_clone, e);
                            }
                        }
                    });
                } else {
                    eprintln!("❌ No API key configured. Please set your API key in Settings.\n");
                }
            }

            // Small sleep to prevent busy-waiting
            std::thread::sleep(Duration::from_millis(50));
        }
    });

    // Start Tauri app on main thread
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--minimized"]),
        ))
        .setup(|app| {
            // Setup system tray
            setup_system_tray(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::save_api_key,
            commands::get_api_key,
            commands::delete_api_key,
            commands::test_api_key,
            commands::get_templates,
            commands::save_template,
            commands::delete_template,
            commands::get_default_model,
            commands::set_default_model,
            commands::reload_hotkeys,
            get_history,
            get_history_page,
            clear_history,
            search_history,
            get_history_count,
            show_main_window,
            hide_main_window,
            toggle_main_window,
            is_autostart_enabled,
            enable_autostart,
            disable_autostart,
            migration::check_migration_needed,
            migration::perform_migration,
            migration::skip_migration,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Setup system tray with menu
fn setup_system_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder};

    // Create menu items
    let show_item = MenuItemBuilder::with_id("show", "Show Window").build(app)?;
    let hide_item = MenuItemBuilder::with_id("hide", "Hide Window").build(app)?;
    let separator1 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let settings_item = MenuItemBuilder::with_id("settings", "Settings").build(app)?;
    let history_item = MenuItemBuilder::with_id("history", "History").build(app)?;
    let separator2 = tauri::menu::PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit").build(app)?;

    // Build menu
    let menu = MenuBuilder::new(app)
        .item(&show_item)
        .item(&hide_item)
        .item(&separator1)
        .item(&settings_item)
        .item(&history_item)
        .item(&separator2)
        .item(&quit_item)
        .build()?;

    // Create tray icon
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| {
            let window = app.get_webview_window("main");

            match event.id().as_ref() {
                "show" => {
                    if let Some(window) = window {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "hide" => {
                    if let Some(window) = window {
                        let _ = window.hide();
                    }
                }
                "settings" => {
                    if let Some(window) = window {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.eval("window.location.href = '/settings'");
                    }
                }
                "history" => {
                    if let Some(window) = window {
                        let _ = window.show();
                        let _ = window.set_focus();
                        let _ = window.eval("window.location.href = '/history'");
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // Toggle window on tray icon click
            if let tauri::tray::TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    println!("✓ System tray initialized");

    Ok(())
}

/// Try to load API client from keychain
fn load_api_client() -> Option<ApiClient> {
    use keyring::Entry;

    const SERVICE_NAME: &str = "com.hotkey-prompt-refiner";
    const KEYRING_USER: &str = "api_key";

    let entry = Entry::new(SERVICE_NAME, KEYRING_USER).ok()?;
    let api_key = entry.get_password().ok()?;

    Some(ApiClient::new(api_key, None))
}

/// Register hotkeys for all templates
fn register_template_hotkeys(hotkey_manager: &HotkeyManager) -> Result<(), String> {
    use commands::get_templates;

    let templates = get_templates()?;
    let mut registered = 0;

    for template in templates {
        if let Some(hotkey) = template.hotkey {
            match hotkey_manager.register_hotkey(template.id.clone(), &hotkey) {
                Ok(()) => registered += 1,
                Err(e) => eprintln!("Warning: Failed to register hotkey for '{}': {}", template.name, e),
            }
        }
    }

    println!("✓ Registered {} template hotkey(s)", registered);
    Ok(())
}

/// Execute a template's workflow
async fn execute_template_workflow(
    template_id: &str,
    api_client: &ApiClient,
) -> Result<(), String> {
    use commands::get_templates;
    use clipboard::ClipboardHandler;
    use paste::PasteHandler;

    // Get the template
    let templates = get_templates()?;
    let template = templates
        .iter()
        .find(|t| t.id == template_id)
        .ok_or_else(|| format!("Template '{}' not found", template_id))?;

    println!("📋 Processing with template: {}", template.name);

    // Step 1: Read clipboard
    let mut clipboard = ClipboardHandler::new()
        .map_err(|e| format!("Failed to access clipboard: {}", e))?;

    let clipboard_text = clipboard
        .get_text()
        .map_err(|e| format!("Failed to read clipboard: {}", e))?;

    println!("  📋 Clipboard content: {} chars", clipboard_text.len());

    println!("  🤖 Calling Claude API...");

    // Step 2 & 3: Call API with template and clipboard text
    let result = api_client
        .process_text(&template.prompt, &clipboard_text)
        .await
        .map_err(|e| format!("API call failed: {}", e))?;

    println!("  ✓ API response: {} chars", result.len());

    // Step 4: Paste result
    let mut paste_handler = PasteHandler::new()
        .map_err(|e| format!("Failed to create paste handler: {}", e))?;

    paste_handler
        .paste_text(&result)
        .map_err(|e| format!("Failed to paste result: {}", e))?;

    println!("  ✓ Result pasted!");

    // Step 5: Record history entry
    let history_entry = models::HistoryEntry::new(
        template.name.clone(),
        clipboard_text,
        result,
    );
    HISTORY.add_entry(history_entry);

    println!("  ✓ History recorded!");

    Ok(())
}
