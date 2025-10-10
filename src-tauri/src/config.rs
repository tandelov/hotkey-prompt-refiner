use serde::Deserialize;
use std::fs;
use std::path::Path;

const DEFAULT_PROMPT_TEMPLATE: &str = r#"Refine and improve the following prompt to be more clear, concise, and effective:

{clipboard_text}

Provide only the refined prompt without explanation."#;

#[derive(Debug)]
pub enum ConfigError {
    MissingApiKey,
    TemplateReadError(std::io::Error),
    TomlParseError(toml::de::Error),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::MissingApiKey => {
                write!(f, "ANTHROPIC_API_KEY not found in environment or .env file")
            }
            ConfigError::TemplateReadError(e) => {
                write!(f, "Failed to read prompt template: {}", e)
            }
            ConfigError::TomlParseError(e) => {
                write!(f, "Failed to parse config.toml: {}", e)
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Deserialize)]
struct TomlConfig {
    #[serde(default)]
    hotkey: HotkeyConfig,
    #[serde(default)]
    api: ApiConfig,
    #[serde(default)]
    prompt: PromptConfig,
}

#[derive(Debug, Deserialize)]
struct HotkeyConfig {
    #[serde(default = "default_modifiers")]
    modifiers: String,
    #[serde(default = "default_key")]
    key: String,
}

#[derive(Debug, Deserialize)]
struct ApiConfig {
    #[serde(default = "default_model")]
    model: String,
    #[serde(default = "default_timeout")]
    timeout_seconds: u64,
}

#[derive(Debug, Deserialize)]
struct PromptConfig {
    #[serde(default = "default_template_file")]
    template_file: String,
}

fn default_modifiers() -> String { "cmd+shift".to_string() }
fn default_key() -> String { "BracketRight".to_string() }
fn default_model() -> String { "claude-3-5-haiku-20241022".to_string() }
fn default_timeout() -> u64 { 30 }
fn default_template_file() -> String { "prompt_template.txt".to_string() }

impl Default for HotkeyConfig {
    fn default() -> Self {
        Self {
            modifiers: default_modifiers(),
            key: default_key(),
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            timeout_seconds: default_timeout(),
        }
    }
}

impl Default for PromptConfig {
    fn default() -> Self {
        Self {
            template_file: default_template_file(),
        }
    }
}

pub struct Config {
    pub api_key: String,
    pub prompt_template: String,
    pub model: String,
    pub timeout_seconds: u64,
    pub hotkey_modifiers: String,
    pub hotkey_key: String,
}

impl Config {
    /// Load configuration from environment and files
    pub fn load() -> Result<Self, ConfigError> {
        // Load .env file if present (already done in main, but safe to call again)
        dotenv::dotenv().ok();

        // Load API key from environment
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| ConfigError::MissingApiKey)?;

        // Load TOML config or use defaults
        let toml_config = Self::load_toml_config();

        // Load prompt template from file specified in config
        let prompt_template = Self::load_prompt_template(&toml_config.prompt.template_file);

        Ok(Config {
            api_key,
            prompt_template,
            model: toml_config.api.model,
            timeout_seconds: toml_config.api.timeout_seconds,
            hotkey_modifiers: toml_config.hotkey.modifiers,
            hotkey_key: toml_config.hotkey.key,
        })
    }

    /// Load TOML configuration file or use defaults
    fn load_toml_config() -> TomlConfig {
        let config_path = "config.toml";

        if Path::new(config_path).exists() {
            match fs::read_to_string(config_path) {
                Ok(content) => match toml::from_str(&content) {
                    Ok(config) => {
                        println!("✓ Loaded configuration from {}", config_path);
                        return config;
                    }
                    Err(e) => {
                        println!("⚠ Warning: Could not parse {}: {}", config_path, e);
                        println!("  Using default configuration");
                    }
                },
                Err(e) => {
                    println!("⚠ Warning: Could not read {}: {}", config_path, e);
                    println!("  Using default configuration");
                }
            }
        } else {
            println!("ℹ Using default configuration (create {} to customize)", config_path);
        }

        // Return default config
        TomlConfig {
            hotkey: HotkeyConfig::default(),
            api: ApiConfig::default(),
            prompt: PromptConfig::default(),
        }
    }

    /// Load prompt template from file, or use default if not found
    fn load_prompt_template(template_path: &str) -> String {
        if Path::new(template_path).exists() {
            match fs::read_to_string(template_path) {
                Ok(content) => {
                    println!("✓ Loaded prompt template from {}", template_path);
                    content
                }
                Err(e) => {
                    println!("⚠ Warning: Could not read {}: {}", template_path, e);
                    println!("  Using default prompt template");
                    DEFAULT_PROMPT_TEMPLATE.to_string()
                }
            }
        } else {
            println!("ℹ Using default prompt template (create {} to customize)", template_path);
            DEFAULT_PROMPT_TEMPLATE.to_string()
        }
    }

    /// Validate that the configuration is usable
    pub fn validate(&self) -> Result<(), String> {
        // Validate API key format (basic check)
        if self.api_key.is_empty() {
            return Err("API key is empty".to_string());
        }

        if !self.api_key.starts_with("sk-") {
            return Err("API key should start with 'sk-'".to_string());
        }

        // Validate prompt template contains placeholder
        if !self.prompt_template.contains("{clipboard_text}") {
            return Err("Prompt template must contain {clipboard_text} placeholder".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_template_has_placeholder() {
        assert!(DEFAULT_PROMPT_TEMPLATE.contains("{clipboard_text}"));
    }

    #[test]
    fn test_config_validate_checks_api_key_format() {
        let config = Config {
            api_key: "invalid".to_string(),
            prompt_template: DEFAULT_PROMPT_TEMPLATE.to_string(),
        };
        assert!(config.validate().is_err());

        let valid_config = Config {
            api_key: "sk-test-key".to_string(),
            prompt_template: DEFAULT_PROMPT_TEMPLATE.to_string(),
        };
        assert!(valid_config.validate().is_ok());
    }

    #[test]
    fn test_config_validate_checks_placeholder() {
        let config = Config {
            api_key: "sk-test".to_string(),
            prompt_template: "Invalid template without placeholder".to_string(),
        };
        assert!(config.validate().is_err());
    }
}
