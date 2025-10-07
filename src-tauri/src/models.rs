use serde::{Deserialize, Serialize};

/// Template for prompt processing with optional hotkey binding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    /// Unique identifier for the template
    pub id: String,
    /// Human-readable name for the template
    pub name: String,
    /// Optional description of what the template does
    #[serde(default)]
    pub description: String,
    /// The prompt template text
    pub prompt: String,
    /// Optional hotkey binding (e.g., "Cmd+Shift+]")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotkey: Option<String>,
    /// ISO 8601 timestamp of creation
    pub created_at: String,
}

impl Template {
    /// Create a new template with the given parameters
    pub fn new(name: String, description: String, prompt: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            description,
            prompt,
            hotkey: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Application configuration persisted to disk
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// List of user-defined templates
    #[serde(default)]
    pub templates: Vec<Template>,
    /// Default Claude model to use (e.g., "claude-3-5-sonnet-20241022")
    #[serde(default = "default_model")]
    pub default_model: String,
}

fn default_model() -> String {
    "claude-3-5-sonnet-20241022".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            templates: Vec::new(),
            default_model: default_model(),
        }
    }
}

/// History entry for a single prompt processing operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// Name of the template used
    pub template_name: String,
    /// First 100 chars of source text
    pub source_preview: String,
    /// First 100 chars of result text
    pub result_preview: String,
    /// Full source text
    pub full_source: String,
    /// Full result text
    pub full_result: String,
}

impl HistoryEntry {
    /// Create a new history entry
    pub fn new(
        template_name: String,
        source: String,
        result: String,
    ) -> Self {
        let source_preview = if source.len() > 100 {
            format!("{}...", &source[..100])
        } else {
            source.clone()
        };

        let result_preview = if result.len() > 100 {
            format!("{}...", &result[..100])
        } else {
            result.clone()
        };

        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            template_name,
            source_preview,
            result_preview,
            full_source: source,
            full_result: result,
        }
    }
}
