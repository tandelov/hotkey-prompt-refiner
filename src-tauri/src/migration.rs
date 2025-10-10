use crate::commands::{save_api_key, save_template};
use crate::models::Template;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

const MIGRATION_FLAG_FILE: &str = ".migration_completed";

/// Check if migration has already been completed
pub fn is_migration_completed() -> Result<bool, String> {
    let flag_path = get_migration_flag_path()?;
    Ok(flag_path.exists())
}

/// Mark migration as completed
pub fn mark_migration_completed() -> Result<(), String> {
    let flag_path = get_migration_flag_path()?;

    // Create parent directory if it doesn't exist
    if let Some(parent) = flag_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }

    fs::write(&flag_path, "")
        .map_err(|e| format!("Failed to create migration flag: {}", e))
}

/// Get the path to the migration flag file
fn get_migration_flag_path() -> Result<PathBuf, String> {
    let config_dir = ProjectDirs::from("com", "hotkey-prompt-refiner", "Hotkey Prompt Refiner")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
        .ok_or_else(|| "Failed to determine config directory".to_string())?;

    Ok(config_dir.join(MIGRATION_FLAG_FILE))
}

/// Detected .env file with parsed contents
#[derive(Debug, Clone)]
pub struct DetectedEnvFile {
    pub path: PathBuf,
    pub api_key: Option<String>,
    pub prompt_template: Option<String>,
}

/// Detect .env files in common locations
pub fn detect_env_files() -> Vec<DetectedEnvFile> {
    let mut detected = Vec::new();

    // Check current directory
    if let Some(env_file) = check_env_file(PathBuf::from(".env")) {
        detected.push(env_file);
    }

    // Check user home directory
    if let Some(home_dir) = dirs::home_dir() {
        if let Some(env_file) = check_env_file(home_dir.join(".env")) {
            detected.push(env_file);
        }

        // Check legacy config directory
        if let Some(env_file) = check_env_file(home_dir.join(".hotkey-prompt-refiner/.env")) {
            detected.push(env_file);
        }
    }

    // Check project root (one level up from where binary might be running)
    if let Ok(current_dir) = std::env::current_dir() {
        if let Some(parent) = current_dir.parent() {
            if let Some(env_file) = check_env_file(parent.join(".env")) {
                detected.push(env_file);
            }
        }
    }

    detected
}

/// Check if .env file exists and parse it
fn check_env_file(path: PathBuf) -> Option<DetectedEnvFile> {
    if !path.exists() {
        return None;
    }

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return None,
    };

    let api_key = parse_env_var(&contents, "ANTHROPIC_API_KEY");
    let prompt_template_path = parse_env_var(&contents, "PROMPT_TEMPLATE");

    // Try to load prompt template from file if path is specified
    let prompt_template = prompt_template_path
        .and_then(|path_str| {
            let template_path = if PathBuf::from(&path_str).is_absolute() {
                PathBuf::from(path_str)
            } else {
                // Resolve relative to .env file location
                path.parent()
                    .map(|p| p.join(&path_str))
                    .unwrap_or_else(|| PathBuf::from(path_str))
            };

            fs::read_to_string(template_path).ok()
        });

    // Only return if we found at least one useful value
    if api_key.is_some() || prompt_template.is_some() {
        Some(DetectedEnvFile {
            path,
            api_key,
            prompt_template,
        })
    } else {
        None
    }
}

/// Parse a single environment variable from .env file contents
fn parse_env_var(contents: &str, var_name: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();

        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Parse KEY=VALUE format
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            if key == var_name {
                let value = value.trim();

                // Remove quotes if present
                let value = value
                    .strip_prefix('"').and_then(|v| v.strip_suffix('"'))
                    .or_else(|| value.strip_prefix('\'').and_then(|v| v.strip_suffix('\'')))
                    .unwrap_or(value);

                return Some(value.to_string());
            }
        }
    }

    None
}

/// Import settings from detected .env file
pub async fn import_from_env(env_file: &DetectedEnvFile) -> Result<ImportResult, String> {
    let mut result = ImportResult::default();

    // Import API key
    if let Some(api_key) = &env_file.api_key {
        save_api_key(api_key.clone())?;
        result.api_key_imported = true;
    }

    // Import prompt template as a default template
    if let Some(prompt) = &env_file.prompt_template {
        let template = Template::new(
            "Imported from CLI".to_string(),
            "Automatically imported from .env configuration".to_string(),
            prompt.clone(),
        );

        save_template(template)?;
        result.template_imported = true;
    }

    Ok(result)
}

/// Result of import operation
#[derive(Debug, Default, serde::Serialize)]
pub struct ImportResult {
    pub api_key_imported: bool,
    pub template_imported: bool,
}

impl ImportResult {
    pub fn anything_imported(&self) -> bool {
        self.api_key_imported || self.template_imported
    }
}

/// Tauri command to check if migration is needed
#[tauri::command]
pub fn check_migration_needed() -> Result<MigrationStatus, String> {
    // Check if migration was already completed
    if is_migration_completed()? {
        return Ok(MigrationStatus {
            needed: false,
            env_files: Vec::new(),
        });
    }

    // Detect .env files
    let env_files = detect_env_files();

    Ok(MigrationStatus {
        needed: !env_files.is_empty(),
        env_files: env_files.into_iter().map(|e| EnvFileInfo {
            path: e.path.to_string_lossy().to_string(),
            has_api_key: e.api_key.is_some(),
            has_template: e.prompt_template.is_some(),
        }).collect(),
    })
}

/// Tauri command to perform migration
#[tauri::command]
pub async fn perform_migration(env_file_path: String) -> Result<ImportResult, String> {
    // Find the env file with this path
    let env_files = detect_env_files();
    let env_file = env_files.into_iter()
        .find(|e| e.path.to_string_lossy() == env_file_path)
        .ok_or_else(|| "Env file not found".to_string())?;

    // Perform import
    let result = import_from_env(&env_file).await?;

    // Mark migration as completed
    mark_migration_completed()?;

    Ok(result)
}

/// Tauri command to skip migration
#[tauri::command]
pub fn skip_migration() -> Result<(), String> {
    mark_migration_completed()
}

/// Migration status information for frontend
#[derive(Debug, serde::Serialize)]
pub struct MigrationStatus {
    pub needed: bool,
    pub env_files: Vec<EnvFileInfo>,
}

/// Information about a detected .env file
#[derive(Debug, serde::Serialize)]
pub struct EnvFileInfo {
    pub path: String,
    pub has_api_key: bool,
    pub has_template: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_env_var_simple() {
        let contents = "ANTHROPIC_API_KEY=sk-test-123\n";
        assert_eq!(
            parse_env_var(contents, "ANTHROPIC_API_KEY"),
            Some("sk-test-123".to_string())
        );
    }

    #[test]
    fn test_parse_env_var_with_quotes() {
        let contents = r#"ANTHROPIC_API_KEY="sk-test-123""#;
        assert_eq!(
            parse_env_var(contents, "ANTHROPIC_API_KEY"),
            Some("sk-test-123".to_string())
        );
    }

    #[test]
    fn test_parse_env_var_with_single_quotes() {
        let contents = "ANTHROPIC_API_KEY='sk-test-123'";
        assert_eq!(
            parse_env_var(contents, "ANTHROPIC_API_KEY"),
            Some("sk-test-123".to_string())
        );
    }

    #[test]
    fn test_parse_env_var_with_comments() {
        let contents = r#"
# This is a comment
ANTHROPIC_API_KEY=sk-test-123
# Another comment
OTHER_VAR=value
"#;
        assert_eq!(
            parse_env_var(contents, "ANTHROPIC_API_KEY"),
            Some("sk-test-123".to_string())
        );
    }

    #[test]
    fn test_parse_env_var_not_found() {
        let contents = "OTHER_VAR=value\n";
        assert_eq!(parse_env_var(contents, "ANTHROPIC_API_KEY"), None);
    }

    #[test]
    fn test_parse_env_var_empty_lines() {
        let contents = "\n\nANTHROPIC_API_KEY=sk-test-123\n\n";
        assert_eq!(
            parse_env_var(contents, "ANTHROPIC_API_KEY"),
            Some("sk-test-123".to_string())
        );
    }
}
