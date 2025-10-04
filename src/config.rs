use std::fs;
use std::path::Path;

const DEFAULT_PROMPT_TEMPLATE: &str = r#"Refine and improve the following prompt to be more clear, concise, and effective:

{clipboard_text}

Provide only the refined prompt without explanation."#;

#[derive(Debug)]
pub enum ConfigError {
    MissingApiKey,
    TemplateReadError(std::io::Error),
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
        }
    }
}

impl std::error::Error for ConfigError {}

pub struct Config {
    pub api_key: String,
    pub prompt_template: String,
}

impl Config {
    /// Load configuration from environment and files
    pub fn load() -> Result<Self, ConfigError> {
        // Load .env file if present (already done in main, but safe to call again)
        dotenv::dotenv().ok();

        // Load API key from environment
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .map_err(|_| ConfigError::MissingApiKey)?;

        // Load prompt template from file or use default
        let prompt_template = Self::load_prompt_template();

        Ok(Config {
            api_key,
            prompt_template,
        })
    }

    /// Load prompt template from file, or use default if not found
    fn load_prompt_template() -> String {
        let template_path = "prompt_template.txt";

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
