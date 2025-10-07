# Project Structure: Hotkey Prompt Refiner

## Directory Overview

```
hotkey-prompt-refiner/
├── .claude/                    # Claude Code configuration and project management
│   ├── agents/                 # Custom agent configurations
│   ├── commands/               # Slash command definitions
│   ├── context/               # Project context documentation (this file)
│   ├── epics/                 # Epic and task tracking
│   ├── hooks/                 # Git hooks and automation
│   ├── prds/                  # Product requirement documents
│   ├── rules/                 # Development rules and standards
│   ├── scripts/               # Automation scripts
│   └── settings.local.json    # Local Claude Code settings
├── src/                       # Rust source code
│   ├── main.rs                # Application entry point
│   ├── config.rs              # Configuration management
│   ├── hotkey.rs              # Global hotkey system
│   ├── clipboard.rs           # Clipboard integration
│   ├── anthropic.rs           # API client implementation
│   ├── paste.rs               # Auto-paste functionality
│   └── workflow.rs            # Workflow orchestration
├── target/                    # Build artifacts (gitignored)
├── Cargo.toml                 # Rust project manifest
├── Cargo.lock                 # Dependency lock file
├── README.md                  # User documentation
├── CLAUDE.md                  # AI assistant guidance
├── prompt_template.txt.example # Example prompt template
└── .env                       # Environment variables (gitignored)
```

## Source Code Structure (`src/`)

### Entry Point

#### `main.rs` (121 lines)
**Purpose**: Application entry point and event loop coordination

**Key Components**:
- Main function with event loop setup
- Thread coordination (main thread for hotkeys, worker for async)
- Configuration loading and validation
- Hotkey registration and event handling
- Worker thread with Tokio runtime for API calls

**Workflow**:
1. Load configuration from environment
2. Initialize hotkey manager
3. Register global hotkey
4. Create communication channel between threads
5. Spawn worker thread with async runtime
6. Run event loop on main thread (required for macOS)
7. Handle hotkey events and trigger workflow

**Dependencies**:
- All other modules (config, hotkey, clipboard, anthropic, paste, workflow)
- `tokio` for async runtime
- `tao` for event loop
- `global-hotkey` for event handling

### Core Modules

#### `config.rs` (~130 lines)
**Purpose**: Configuration loading, validation, and management

**Public API**:
```rust
pub struct Config {
    pub api_key: String,
    pub prompt_template: String,
}

pub enum ConfigError {
    MissingApiKey,
    TemplateReadError(std::io::Error),
}

impl Config {
    pub fn load() -> Result<Self, ConfigError>
    pub fn validate(&self) -> Result<(), String>
}
```

**Responsibilities**:
- Load `ANTHROPIC_API_KEY` from environment
- Load prompt template from `prompt_template.txt` or use default
- Validate API key format (starts with "sk-")
- Validate template contains `{clipboard_text}` placeholder
- Provide clear error messages

**Constants**:
- `DEFAULT_PROMPT_TEMPLATE`: Built-in fallback template

**Tests**: Unit tests for validation logic

#### `hotkey.rs` (~126 lines)
**Purpose**: Global hotkey registration and management

**Public API**:
```rust
pub struct HotkeyManager {
    manager: GlobalHotKeyManager,
    hotkey: HotKey,
}

pub enum HotkeyError {
    ManagerCreationFailed(String),
    RegistrationFailed(String),
    AlreadyInUse,
}

impl HotkeyManager {
    pub fn new() -> Result<Self, HotkeyError>
    pub fn register(&self) -> Result<(), HotkeyError>
    pub fn unregister(&self) -> Result<(), HotkeyError>
    pub fn hotkey_description(&self) -> String
}
```

**Platform-Specific**:
- macOS: Cmd+Shift+] (SUPER + SHIFT + BracketRight)
- Windows/Linux: Ctrl+Shift+] (CONTROL + SHIFT + BracketRight)

**Responsibilities**:
- Create global hotkey manager
- Register system-wide hotkey
- Unregister on drop (cleanup)
- Provide user-friendly hotkey description

**Tests**: Unit tests for manager creation and description

#### `clipboard.rs` (~50 lines)
**Purpose**: Clipboard text capture

**Public API**:
```rust
pub struct ClipboardHandler;

pub enum ClipboardError {
    ReadError(arboard::Error),
    EmptyClipboard,
}

impl ClipboardHandler {
    pub fn new() -> Result<Self, ClipboardError>
    pub fn get_text(&mut self) -> Result<String, ClipboardError>
}
```

**Responsibilities**:
- Create clipboard instance via `arboard`
- Read text content from clipboard
- Handle empty clipboard gracefully
- Convert clipboard errors to custom error type

**Dependencies**: `arboard` crate

#### `anthropic.rs` (~150 lines)
**Purpose**: Direct HTTP API integration with Anthropic Claude

**Public API**:
```rust
pub struct ApiClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
}

pub enum ApiError {
    NetworkError(reqwest::Error),
    InvalidResponse(String),
    ApiFailure { status: u16, message: String },
}

impl ApiClient {
    pub fn new(api_key: String, model: Option<String>) -> Self
    pub async fn process_text(&self, prompt_template: &str, clipboard_text: &str) -> Result<String, ApiError>
}
```

**Implementation Details**:
- Direct HTTP POST to `https://api.anthropic.com/v1/messages`
- Headers: `x-api-key`, `anthropic-version`, `content-type`
- Request/response serialization with `serde_json`
- Default model: `claude-3-5-haiku-20241022`
- Error handling for HTTP errors, malformed responses, API errors

**Request Structure**:
```rust
#[derive(Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}
```

**Response Parsing**:
- Extracts text content from nested JSON structure
- Handles multiple content blocks (takes first text block)

**Dependencies**: `reqwest`, `serde`, `serde_json`

#### `paste.rs` (~60 lines)
**Purpose**: Auto-paste functionality via keyboard simulation

**Public API**:
```rust
pub struct PasteHandler;

pub enum PasteError {
    SimulationError(String),
    ClipboardError(arboard::Error),
}

impl PasteHandler {
    pub fn new() -> Result<Self, PasteError>
    pub fn paste_text(&mut self, text: &str) -> Result<(), PasteError>
}
```

**Implementation**:
1. Copy text to clipboard
2. Simulate Cmd+V (macOS) or Ctrl+V (Windows/Linux) keypress
3. Brief delay for reliability

**Platform-Specific**:
- macOS: `Key::Meta + Key::Layout('v')`
- Windows/Linux: `Key::Control + Key::Layout('v')`

**Dependencies**: `enigo`, `arboard`, `std::thread::sleep`

**Notes**: Requires Accessibility permissions on macOS

#### `workflow.rs` (~80 lines)
**Purpose**: Complete workflow orchestration

**Public API**:
```rust
pub enum WorkflowError {
    ClipboardError(ClipboardError),
    ApiError(ApiError),
    PasteError(PasteError),
}

pub async fn execute_workflow_with_logging(
    config: &Config,
    api_client: &ApiClient,
) -> Result<(), WorkflowError>
```

**Workflow Steps**:
1. Read clipboard text
2. Log clipboard content (first 100 chars)
3. Call API with clipboard text and prompt template
4. Log API response
5. Auto-paste response
6. Log success

**Error Handling**:
- Converts errors from each step to `WorkflowError`
- Provides context for each failure point
- Logs errors to console

**Dependencies**: All core modules

## Configuration Files

### `Cargo.toml`
**Purpose**: Rust project manifest

**Sections**:
- `[package]`: Project metadata (name, version, edition)
- `[dependencies]`: All external crates with versions
- `[profile.release]`: Build optimizations for release

**Key Settings**:
```toml
edition = "2024"
opt-level = "z"     # Size optimization
lto = true          # Link-time optimization
strip = true        # Strip symbols
codegen-units = 1   # Better optimization
```

### `Cargo.lock`
**Purpose**: Dependency lock file (auto-generated)
**Version Control**: Committed (ensures reproducible builds)

### `.env` (gitignored)
**Purpose**: Environment variables
**Contents**:
```
ANTHROPIC_API_KEY=sk-ant-...
```

### `prompt_template.txt` (optional, gitignored)
**Purpose**: User's custom prompt template
**Format**: Plain text with `{clipboard_text}` placeholder
**Fallback**: Default template in `config.rs` if not present

### `prompt_template.txt.example`
**Purpose**: Example template file
**Usage**: Copy to `prompt_template.txt` and customize

## Documentation Files

### `README.md` (~350 lines)
**Purpose**: User-facing documentation

**Sections**:
- Project overview and features
- Installation instructions (macOS, Windows, Linux)
- Configuration guide (API key, templates)
- Usage instructions with examples
- Troubleshooting (7+ common issues)
- Platform-specific notes
- Development guide
- Performance characteristics
- Architecture overview

**Audience**: End users and contributors

### `CLAUDE.md` (~200 lines)
**Purpose**: AI assistant guidance

**Sections**:
- Project overview
- Architecture and core components
- API implementation guidelines (critical: no SDK crates)
- Development commands
- Configuration details
- Performance requirements
- Security guidelines

**Audience**: Claude Code and other AI assistants

### `.claude/context/` (Context documentation)
**Purpose**: Comprehensive project knowledge base

**Files**:
- `README.md`: Context system overview
- `project-brief.md`: Scope, goals, success criteria
- `project-vision.md`: Long-term vision and roadmap
- `project-overview.md`: Features, status, metrics
- `progress.md`: Current status and next steps
- `tech-context.md`: Technologies, dependencies, tools
- `project-structure.md`: This file
- `system-patterns.md`: Architectural patterns (to be created)
- `project-style-guide.md`: Coding standards (to be created)

## Claude Code Integration (`.claude/`)

### `epics/hotkey-prompt-refiner/`
**Purpose**: Epic and task tracking

**Files**:
- `epic.md`: Epic overview, architecture, task list
- `2.md` - `11.md`: Individual task definitions
- `dependencies.md`: Task dependency graph

**Format**: Markdown with YAML frontmatter

### `prds/`
**Purpose**: Product requirement documents
**File**: `hotkey-prompt-refiner.md` - Full PRD

### `commands/`
**Purpose**: Custom slash commands for Claude Code
**Categories**:
- `pm/`: Project management commands
- `context/`: Context management commands
- `testing/`: Test execution commands

### `agents/`
**Purpose**: Custom agent configurations
**Agents**:
- `code-analyzer.md`: Code review and bug detection
- `file-analyzer.md`: Log file analysis
- `parallel-worker.md`: Parallel task execution
- `test-runner.md`: Test execution and analysis

### `rules/`
**Purpose**: Development rules and standards
**Topics**:
- Agent coordination
- Branch operations
- GitHub integration
- Path standards
- Test execution

### `scripts/`
**Purpose**: Automation scripts
**Categories**:
- `pm/`: Project management scripts
- `test-and-log.sh`: Test execution with logging
- Path validation and fixing scripts

## Build Artifacts (`target/`)

### Structure
```
target/
├── debug/                  # Debug builds
│   └── hotkey-prompt-refiner
├── release/                # Release builds
│   └── hotkey-prompt-refiner
└── [intermediate files]    # Object files, dependencies, etc.
```

**Note**: Entire `target/` directory is gitignored

## File Naming Conventions

### Source Files
- **Format**: `snake_case.rs`
- **Examples**: `config.rs`, `hotkey.rs`, `clipboard.rs`
- **Rule**: One module per file, named after primary type

### Documentation
- **Format**: `SCREAMING_CASE.md` for root, `kebab-case.md` for subdirs
- **Examples**: `README.md`, `CLAUDE.md`, `project-brief.md`

### Configuration
- **Format**: Varies by file type
- **Examples**: `Cargo.toml`, `.env`, `prompt_template.txt`

## Import/Module Structure

### Module Declaration (in `main.rs`)
```rust
mod anthropic;
mod clipboard;
mod config;
mod hotkey;
mod paste;
mod workflow;
```

### Import Pattern
```rust
use config::Config;
use hotkey::HotkeyManager;
use workflow::execute_workflow_with_logging;
```

**Convention**: Import types/functions explicitly, not modules

## Code Organization Principles

### Separation of Concerns
- Each module has single, well-defined responsibility
- No circular dependencies
- Clear module boundaries

### Error Handling
- Each module defines custom error enum
- Errors implement `std::error::Error` and `Display`
- `From` implementations for error conversion

### Testing
- Unit tests inline with modules (`#[cfg(test)] mod tests`)
- Integration tests in `tests/` directory (planned)

### Platform Abstraction
- Platform-specific code in conditional compilation blocks
- `#[cfg(target_os = "macos")]`, `#[cfg(not(target_os = "macos"))]`
- Platform differences isolated to minimal code sections

## File Size Guidelines

### Current Sizes
- `main.rs`: ~120 lines
- `config.rs`: ~130 lines
- `hotkey.rs`: ~125 lines
- `anthropic.rs`: ~150 lines
- `clipboard.rs`: ~50 lines
- `paste.rs`: ~60 lines
- `workflow.rs`: ~80 lines

**Total Source**: ~800 LOC

### Guidelines
- **Target**: Keep modules under 200 lines
- **Rationale**: Easier to understand, test, maintain
- **Action**: If module grows beyond 200 lines, consider splitting

## Adding New Features

### Where to Add Code

**New API Provider** (e.g., OpenAI):
- Create `src/openai.rs` module
- Define similar `ApiClient` interface
- Add provider selection in `config.rs`

**New Configuration Option**:
- Add field to `Config` struct in `config.rs`
- Update loading logic
- Update validation
- Document in README

**New Hotkey**:
- Modify `hotkey.rs` to support multiple hotkeys
- Add configuration in `config.rs`
- Update event handling in `main.rs`

**New Workflow Step**:
- Add function to `workflow.rs`
- Integrate into `execute_workflow_with_logging`
- Add error handling

## Dependencies on File Structure

### Hard Dependencies
- `Cargo.toml` must exist at root
- `src/main.rs` must exist (entry point)
- `.env` or `ANTHROPIC_API_KEY` environment variable required

### Soft Dependencies
- `prompt_template.txt` optional (falls back to default)
- `.claude/` directory optional (Claude Code features)

### Gitignored Files
```
target/             # Build artifacts
.env               # API key
prompt_template.txt # User's custom template
```

## Future Structure Changes

### Planned Additions
- `tests/`: Integration test directory
- `examples/`: Example code and usage
- `docs/`: Additional documentation
- `assets/`: Icons, images for installers
- `scripts/`: Build and release automation

### Potential Refactoring
- Split `workflow.rs` if it grows (pre/post processing)
- Add `providers/` directory for multiple API providers
- Add `ui/` directory if GUI added in future
- Consider workspace structure if project grows significantly

## Summary

The project structure is intentionally simple and flat:
- **7 source modules** with clear responsibilities
- **Minimal configuration files** (Cargo.toml, .env)
- **Comprehensive documentation** (README, CLAUDE.md, .claude/context/)
- **Claude Code integration** for project management

This structure supports:
- Easy navigation for new contributors
- Clear separation of concerns
- Simple build and deployment
- Extensibility for future features
