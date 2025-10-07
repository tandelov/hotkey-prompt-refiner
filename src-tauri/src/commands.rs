use crate::models::{AppConfig, Template};
use directories::ProjectDirs;
use keyring::Entry;
use std::fs;
use std::path::PathBuf;

const SERVICE_NAME: &str = "com.hotkey-prompt-refiner";
const KEYRING_USER: &str = "api_key";

/// Get the application configuration directory
fn get_config_dir() -> Result<PathBuf, String> {
    ProjectDirs::from("com", "hotkey-prompt-refiner", "Hotkey Prompt Refiner")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .ok_or_else(|| "Failed to determine config directory".to_string())
}

/// Get the path to the config file
fn get_config_path() -> Result<PathBuf, String> {
    let config_dir = get_config_dir()?;
    Ok(config_dir.join("config.json"))
}

/// Load the application configuration from disk
fn load_config() -> Result<AppConfig, String> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        return Ok(AppConfig::default());
    }

    let contents = fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    serde_json::from_str(&contents)
        .map_err(|e| format!("Failed to parse config file: {}", e))
}

/// Save the application configuration to disk
fn save_config(config: &AppConfig) -> Result<(), String> {
    let config_dir = get_config_dir()?;

    // Create config directory if it doesn't exist
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("Failed to create config directory: {}", e))?;

    let config_path = get_config_path()?;
    let contents = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(&config_path, contents)
        .map_err(|e| format!("Failed to write config file: {}", e))
}

/// Save API key to system keychain
#[tauri::command]
pub fn save_api_key(api_key: String) -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEYRING_USER)
        .map_err(|e| format!("Failed to access keyring: {}", e))?;

    entry.set_password(&api_key)
        .map_err(|e| format!("Failed to save API key: {}", e))
}

/// Get API key from system keychain
#[tauri::command]
pub fn get_api_key() -> Result<Option<String>, String> {
    let entry = Entry::new(SERVICE_NAME, KEYRING_USER)
        .map_err(|e| format!("Failed to access keyring: {}", e))?;

    match entry.get_password() {
        Ok(password) => Ok(Some(password)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(format!("Failed to retrieve API key: {}", e)),
    }
}

/// Delete API key from system keychain
#[tauri::command]
pub fn delete_api_key() -> Result<(), String> {
    let entry = Entry::new(SERVICE_NAME, KEYRING_USER)
        .map_err(|e| format!("Failed to access keyring: {}", e))?;

    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
        Err(e) => Err(format!("Failed to delete API key: {}", e)),
    }
}

/// Test API key by making a simple request to Anthropic API
#[tauri::command]
pub async fn test_api_key(api_key: String) -> Result<bool, String> {
    let client = reqwest::Client::new();

    // Make a minimal request to test the API key
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&serde_json::json!({
            "model": "claude-3-5-sonnet-20241022",
            "max_tokens": 1,
            "messages": [
                {
                    "role": "user",
                    "content": "Hi"
                }
            ]
        }))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Anthropic API: {}", e))?;

    Ok(response.status().is_success())
}

/// Get all templates
#[tauri::command]
pub fn get_templates() -> Result<Vec<Template>, String> {
    let config = load_config()?;
    Ok(config.templates)
}

/// Save a template (create or update)
#[tauri::command]
pub fn save_template(template: Template) -> Result<(), String> {
    let mut config = load_config()?;

    // Check if template with this ID already exists
    if let Some(index) = config.templates.iter().position(|t| t.id == template.id) {
        // Update existing template
        config.templates[index] = template;
    } else {
        // Add new template
        config.templates.push(template);
    }

    save_config(&config)
}

/// Delete a template by ID
#[tauri::command]
pub fn delete_template(id: String) -> Result<(), String> {
    let mut config = load_config()?;

    config.templates.retain(|t| t.id != id);

    save_config(&config)
}

/// Get the default model
#[tauri::command]
pub fn get_default_model() -> Result<String, String> {
    let config = load_config()?;
    Ok(config.default_model)
}

/// Set the default model
#[tauri::command]
pub fn set_default_model(model: String) -> Result<(), String> {
    let mut config = load_config()?;
    config.default_model = model;
    save_config(&config)
}

/// Reload all hotkeys from templates
/// Returns the number of hotkeys registered
#[tauri::command]
pub fn reload_hotkeys() -> Result<usize, String> {
    // This will be called from the event loop thread
    // For now, return success - actual implementation will be in lib.rs
    let config = load_config()?;
    let hotkey_count = config.templates.iter().filter(|t| t.hotkey.is_some()).count();
    Ok(hotkey_count)
}

// History commands will be implemented in lib.rs to access the global HISTORY store
