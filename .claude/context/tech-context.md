# Technical Context: Hotkey Prompt Refiner

## Technology Stack

### Core Language & Runtime
- **Language**: Rust 2024 Edition
- **Compiler**: rustc 1.80+ (stable channel)
- **Build Tool**: Cargo (standard Rust toolchain)
- **Async Runtime**: Tokio 1.41 (multi-threaded runtime)

### Primary Dependencies

#### System Integration (4 crates)
```toml
global-hotkey = "0.6"     # Cross-platform global hotkey registration
tao = "0.30"              # Event loop for macOS hotkey compatibility
arboard = "3.4"           # Cross-platform clipboard access
enigo = "0.2"             # Keyboard simulation for auto-paste
```

**Rationale**:
- `global-hotkey`: Best-maintained cross-platform hotkey library
- `tao`: Required for macOS main-thread event loop requirement
- `arboard`: Pure Rust, minimal dependencies, cross-platform
- `enigo`: Simple API for keyboard events, works across platforms

#### Async & HTTP (2 crates)
```toml
tokio = { version = "1.41", features = ["rt-multi-thread", "macros"] }
reqwest = { version = "0.12", features = ["json"] }
```

**Rationale**:
- `tokio`: Industry-standard async runtime, well-optimized
- `reqwest`: Most popular Rust HTTP client, built on hyper

#### Data Handling (2 crates)
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**Rationale**:
- `serde`: De facto standard for serialization in Rust
- `serde_json`: Official JSON support for serde

#### Configuration (1 crate)
```toml
dotenv = "0.15"
```

**Rationale**:
- `dotenv`: Simple, standard .env file support

### Total Dependency Count
- **Direct dependencies**: 9 crates
- **Transitive dependencies**: ~80 crates (typical for Rust project with HTTP + GUI event loop)
- **Philosophy**: Minimal direct dependencies, prefer well-maintained crates

### Explicitly Excluded Dependencies

**Anthropic SDK Crates** ❌
- `anthropic-sdk`, `claude-api`, or similar
- **Reason**: Direct HTTP gives full control, smaller binary, no SDK version lock-in

**GUI Frameworks** ❌
- `egui`, `iced`, `druid`, etc.
- **Reason**: CLI-only for minimal footprint, may add later

**Logging Frameworks** ❌
- `env_logger`, `tracing`, etc. (for now)
- **Reason**: Simple console output sufficient for MVP

**Database/Storage** ❌
- `rusqlite`, `sled`, etc.
- **Reason**: No persistent data storage required

## Development Tools

### Required Tools
- **Rust Toolchain**: Install via [rustup](https://rustup.rs/)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**: Included with Rust toolchain
  - Build: `cargo build --release`
  - Run: `cargo run`
  - Test: `cargo test`
  - Format: `cargo fmt`
  - Lint: `cargo clippy`

### Recommended Tools
- **IDE**: VS Code with rust-analyzer extension
- **Debugger**: CodeLLDB extension for VS Code
- **Profiler**: `cargo-flamegraph` for performance analysis
- **Binary Analysis**: `cargo-bloat` for binary size analysis

### Platform-Specific Requirements

#### macOS
- **Xcode Command Line Tools**: For system headers
  ```bash
  xcode-select --install
  ```

#### Windows
- **Visual Studio Build Tools**: MSVC toolchain for linking
  - Download from https://visualstudio.microsoft.com/downloads/
  - Select "Desktop development with C++"

#### Linux
- **Build Essentials**: GCC, make, pkg-config
  ```bash
  # Ubuntu/Debian
  sudo apt install build-essential pkg-config libssl-dev

  # Fedora
  sudo dnf groupinstall "Development Tools"
  sudo dnf install pkg-config openssl-devel
  ```

## Build Configuration

### Release Profile (Cargo.toml)
```toml
[profile.release]
opt-level = "z"       # Optimize for size ("z" = max size optimization)
lto = true            # Link-time optimization (slower builds, smaller binary)
strip = true          # Strip debug symbols
codegen-units = 1     # Single codegen unit for better optimization
```

**Results**:
- Binary size: ~3-4 MB (release build)
- Build time: ~2-3 minutes (full release build)
- Memory usage: <30 MB idle

### Development Profile (Default)
- Optimizations disabled for fast compile times
- Debug symbols included
- Typical build time: ~30 seconds (incremental)

## API Integration

### Anthropic Claude API

#### Endpoint
```
POST https://api.anthropic.com/v1/messages
```

#### Authentication
- **Method**: API key in header
- **Header**: `x-api-key: <API_KEY>`
- **Source**: Environment variable `ANTHROPIC_API_KEY` or `.env` file

#### Request Format
```json
{
  "model": "claude-3-5-haiku-20241022",
  "max_tokens": 1024,
  "messages": [
    {
      "role": "user",
      "content": "<formatted prompt with clipboard text>"
    }
  ]
}
```

#### Response Format
```json
{
  "id": "msg_...",
  "type": "message",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "<AI response>"
    }
  ],
  "model": "claude-3-5-haiku-20241022",
  "usage": {
    "input_tokens": 100,
    "output_tokens": 50
  }
}
```

#### Model Selection
- **Current**: `claude-3-5-haiku-20241022`
- **Rationale**: Fastest, most cost-effective for short prompts
- **Alternative**: `claude-3-5-sonnet-20241022` for complex tasks (future config option)

#### Error Handling
- **401 Unauthorized**: Invalid API key
- **400 Bad Request**: Malformed request
- **429 Too Many Requests**: Rate limiting
- **500+ Server Errors**: Anthropic API issues

All errors mapped to custom `ApiError` enum with user-friendly messages.

## Platform APIs

### macOS

#### Hotkey Event Loop
- **Requirement**: Event loop must run on main thread
- **Solution**: Use `tao::EventLoop` on main thread
- **Communication**: Worker thread via `std::sync::mpsc` channel

#### Accessibility API
- **Purpose**: Required for keyboard simulation (auto-paste)
- **Permission**: System Preferences → Security & Privacy → Accessibility
- **API**: Uses `enigo` which wraps Core Graphics APIs

### Windows

#### Hotkey Registration
- **API**: Win32 `RegisterHotKey`
- **Wrapper**: `global-hotkey` crate handles platform differences

#### Keyboard Simulation
- **API**: Win32 `SendInput`
- **Wrapper**: `enigo` crate

### Linux

#### Hotkey Registration
- **X11**: XGrabKey API
- **Wayland**: Compositor-dependent (limited support)
- **Wrapper**: `global-hotkey` crate

#### Keyboard Simulation
- **X11**: XTest extension
- **Wayland**: wlroots input method protocol
- **Wrapper**: `enigo` crate

## Threading Model

### Main Thread
**Responsibility**: Event loop for global hotkeys

**Platform Requirement**: macOS requires hotkey event loop on main thread

**Flow**:
```rust
fn main() {
    // Initialize hotkey manager
    let hotkey_manager = HotkeyManager::new();

    // Create channel for communication
    let (tx, rx) = mpsc::channel();

    // Spawn worker thread
    std::thread::spawn(move || { /* async work */ });

    // Run event loop on main thread (blocks)
    event_loop.run(move |event, _, control_flow| {
        // Handle hotkey events, send to worker via tx
    });
}
```

### Worker Thread
**Responsibility**: Async operations (API calls, clipboard, paste)

**Runtime**: Tokio async runtime

**Flow**:
```rust
std::thread::spawn(move || {
    let rt = tokio::runtime::Runtime::new();

    while let Ok(()) = rx.recv() {
        rt.block_on(async {
            // 1. Read clipboard
            // 2. Call API
            // 3. Paste response
        });
    }
});
```

## Memory Management

### Allocation Strategy
- **Default Allocator**: System allocator (glibc malloc on Linux, macOS allocator, Windows heap)
- **No Custom Allocator**: Default is sufficient for our use case
- **String Handling**: Use `String` and `&str` appropriately, minimize clones

### Memory Characteristics
- **Idle**: ~25-30 MB RSS (mostly from event loop and HTTP client connection pool)
- **Active**: ~40-50 MB during API call (request/response buffers)
- **Peak**: <100 MB (large clipboard content + API response)

### Memory Safety
- **Rust Guarantees**: No use-after-free, no data races, no buffer overflows
- **Unsafe Code**: Only in dependencies (carefully vetted crates)
- **Our Code**: 100% safe Rust

## Error Handling Strategy

### Error Types
Each module defines custom error enum:

```rust
// Example from config.rs
pub enum ConfigError {
    MissingApiKey,
    TemplateReadError(std::io::Error),
}

impl Error for ConfigError {}
impl Display for ConfigError { /* ... */ }
```

### Error Propagation
- **Internal**: Use `Result<T, ModuleError>` for all fallible functions
- **User-Facing**: Convert to clear error messages on console
- **No Panics**: Never panic on user input or external errors

### Error Recovery
- **Configuration Errors**: Exit with clear message
- **Runtime Errors**: Log to console, continue running
- **API Errors**: Log error, wait for next hotkey press

## Security Considerations

### API Key Security
- **Storage**: Environment variable or `.env` file only
- **Never in Code**: No hardcoded keys, checked in `.gitignore`
- **Never in Logs**: API key sanitized from error messages
- **Transmission**: HTTPS only (enforced by `reqwest`)

### Network Security
- **TLS**: All API communication over HTTPS
- **Certificate Validation**: Enabled by default in `reqwest`
- **No Custom Certificates**: Use system trust store

### Data Privacy
- **Clipboard**: Never persisted to disk
- **API Responses**: Never logged or saved
- **No Telemetry**: No usage tracking or analytics

### Platform Security
- **macOS Accessibility**: User grants explicitly
- **Windows/Linux**: Standard user permissions only
- **No Elevation**: Never requests admin/root

## Performance Characteristics

### Startup Performance
- **Cold Start**: ~100ms (load libs, register hotkey)
- **Memory**: ~25-30 MB after initialization
- **CPU**: <1% idle (event loop is very efficient)

### Runtime Performance
- **Hotkey Detection**: <10ms (hardware interrupt → event handler)
- **Clipboard Read**: <5ms (system API call)
- **API Call**: 1-3 seconds (network + Claude processing)
- **Paste**: <50ms (keyboard simulation)

### Binary Size
- **Release Build**: ~3-4 MB
- **Optimizations**: LTO, strip, opt-level=z
- **Breakdown**: ~40% stdlib, 30% dependencies, 30% our code

### Compile Times
- **Clean Build (Debug)**: ~45 seconds
- **Clean Build (Release)**: ~2-3 minutes
- **Incremental (Debug)**: ~5-10 seconds
- **Incremental (Release)**: ~30-60 seconds

## Testing Strategy

### Unit Tests
- **Location**: Inline with modules (`#[cfg(test)]` mod tests)
- **Coverage**: Core logic in config, hotkey, clipboard modules
- **Run**: `cargo test`

### Integration Tests
- **Location**: `tests/` directory (planned)
- **Scope**: End-to-end workflow tests
- **Challenges**: Testing global hotkeys, clipboard, paste (requires real system)

### Manual Testing
- **Platforms**: macOS (primary), Windows, Linux
- **Scenarios**:
  - Normal operation (copy → hotkey → paste)
  - Error cases (no API key, network failure, empty clipboard)
  - Edge cases (very long text, special characters, multiple rapid invocations)

## Deployment & Distribution

### Build Artifacts
- **macOS**: Single binary (universal binary planned for Intel + Apple Silicon)
- **Windows**: `.exe` (64-bit)
- **Linux**: ELF binary (64-bit)

### Distribution Channels
- **GitHub Releases**: Primary distribution
- **Package Managers** (Future):
  - macOS: Homebrew
  - Windows: Winget, Chocolatey
  - Linux: apt, dnf, AUR

### Installation
- **Current**: Manual download + move to PATH
- **Future**: Platform-specific installers

## External Services

### Anthropic API
- **Service**: Claude AI via REST API
- **Endpoint**: `https://api.anthropic.com`
- **Authentication**: API key (user-provided)
- **Pricing**: Pay-as-you-go (user's responsibility)
- **Rate Limits**: Tier-dependent (1000-4000 RPM)

### Dependencies
- **crates.io**: Rust package registry
- **GitHub**: Source control and releases
- **docs.rs**: Automatic documentation hosting

## Development Workflow

### Setup
```bash
# Clone repository
git clone https://github.com/tandelov/hotkey-prompt-refiner.git
cd hotkey-prompt-refiner

# Set up API key
echo 'ANTHROPIC_API_KEY=sk-ant-...' > .env

# Build
cargo build --release

# Run
./target/release/hotkey-prompt-refiner
```

### Common Commands
```bash
# Build (debug)
cargo build

# Build (release)
cargo build --release

# Run
cargo run

# Test
cargo test

# Format code
cargo fmt

# Lint
cargo clippy

# Check without building
cargo check

# Update dependencies
cargo update

# Clean build artifacts
cargo clean
```

### Version Control
- **VCS**: Git
- **Hosting**: GitHub
- **Branching**: main branch for releases, feature branches for development
- **Commits**: Conventional commits format recommended

## Future Technical Considerations

### Scalability
- Not applicable (single-user desktop app)

### Extensibility
- **Plugin System**: Consider in Phase 4 (2027+)
- **API**: Potential for third-party integrations
- **Templates**: File-based, easy to share and version

### Maintainability
- **Rust Updates**: Follow stable releases, update annually
- **Dependency Updates**: Review and update quarterly
- **Security Updates**: Apply immediately when available

### Technical Debt
- **Current**: Minimal (new project, clean architecture)
- **Monitoring**: Regular code reviews, clippy warnings
- **Paydown**: Continuous refactoring, no "big rewrite"
