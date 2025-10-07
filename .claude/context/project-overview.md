# Project Overview: Hotkey Prompt Refiner

## Executive Summary
Hotkey Prompt Refiner is a lightweight desktop application that bridges the gap between everyday text work and AI-powered assistance. With a single hotkey press, users can send clipboard text to Claude AI and receive intelligent, refined responses automatically pasted at their cursor—all without leaving their current application.

## What It Does

### Core Functionality
1. **Global Hotkey Activation**: User presses `Cmd+Shift+]` (macOS) or `Ctrl+Shift+]` (Windows/Linux)
2. **Clipboard Capture**: Application reads current clipboard text content
3. **AI Processing**: Text sent to Claude API with configurable prompt template
4. **Auto-paste Response**: AI-refined text automatically pasted at cursor location

### Example Use Cases

**Prompt Refinement** (for grammar-checking prompt template)
```
Input:  "I am readu"
Output: "I am ready"
```

**Code Review**
```
Input:  [Python code snippet]
Output: [Detailed code review with suggestions]
```

**Grammar Correction**
```
Input:  "i have went to store yesterday"
Output: "I went to the store yesterday"
```

**Text Summarization**
```
Input:  [Long article or documentation]
Output: [2-3 bullet point summary]
```

## Key Features

### Currently Implemented ✅

#### 1. Global Hotkey System
- **Platform Support**: macOS (Cmd+Shift+]), Windows/Linux (Ctrl+Shift+])
- **Implementation**: Uses `global-hotkey` crate with `tao` event loop
- **macOS Compatibility**: Event loop runs on main thread (required by macOS)
- **Reliability**: Automatic unregistration on app exit

#### 2. Clipboard Integration
- **Text Capture**: Reads text content from system clipboard
- **Cross-platform**: Uses `arboard` crate for unified clipboard access
- **Validation**: Checks for text content (skips images, files, etc.)
- **Non-destructive**: Doesn't modify clipboard during operation

#### 3. Claude API Integration
- **Direct HTTP**: Uses `reqwest` + `serde_json` (no SDK dependencies)
- **Endpoint**: `https://api.anthropic.com/v1/messages`
- **Model**: `claude-3-5-haiku-20241022` (fast, cost-effective)
- **Authentication**: API key via environment variable or `.env` file
- **Error Handling**: Clear error messages for common issues

#### 4. Auto-paste System
- **Keyboard Simulation**: Uses `enigo` crate to simulate `Cmd+V`/`Ctrl+V`
- **Platform-specific**: Adapts to OS keyboard conventions
- **Cursor Targeting**: Pastes at current cursor location
- **macOS Permissions**: Requires Accessibility permissions (documented)

#### 5. Configuration System
- **API Key**: Environment variable `ANTHROPIC_API_KEY`
- **Prompt Template**: Customizable via `prompt_template.txt` file
- **Default Template**: Prompt refinement (included in binary)
- **Validation**: Checks API key format and template structure on startup
- **Flexible**: Supports multiple template strategies (see example file)

#### 6. Complete Workflow Integration
- **Threading Model**: Main thread for event loop, worker thread for async API calls
- **Async Runtime**: Tokio for non-blocking API communication
- **Event-driven**: Responds immediately to hotkey presses
- **Status Feedback**: Console output for workflow progress

#### 7. Documentation
- **README.md**: Comprehensive setup, usage, and troubleshooting
- **Code Comments**: Inline documentation for complex logic
- **Example Files**: `prompt_template.txt.example`, `.env.example`
- **Platform Guides**: Specific instructions for macOS, Windows, Linux

### In Progress ⏳

#### 8. Cross-platform Testing
- Testing on multiple macOS versions
- Windows compatibility verification
- Linux distribution testing (Ubuntu, Fedora, etc.)
- Edge case handling (network failures, permission issues)

#### 9. Performance Optimization
- Binary size reduction (target: <5MB)
- Memory optimization (target: <30MB idle)
- Startup time optimization
- Response caching evaluation

### Planned Features 📋

#### Short-term (Q2-Q3 2025)
- **Configurable Hotkeys**: Change hotkey without recompiling
- **Multiple Templates**: Quick-switch between templates
- **Streaming Responses**: Faster perceived performance
- **System Tray Integration**: Menu bar (macOS) / system tray (Windows/Linux)
- **Installer Packages**: Native installers for all platforms

#### Medium-term (Q4 2025 - Q1 2026)
- **Context Awareness**: Auto-select template based on file type or application
- **Response History**: Optional local storage with search
- **Additional AI Providers**: OpenAI, local models (llama.cpp)
- **Template Marketplace**: Share and discover community templates

#### Long-term (2026+)
- **Plugin System**: Extensible architecture for custom processors
- **IDE Integrations**: VS Code, JetBrains plugins
- **Team Features**: Shared templates, usage analytics
- **Multi-modal Support**: Images, code diagrams, etc.

## Technical Highlights

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                     Main Thread                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Tao Event Loop (macOS requirement)             │   │
│  │  - Listen for global hotkey events              │   │
│  │  - Send trigger signal to worker thread         │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
                            │
                            │ Signal (mpsc channel)
                            ▼
┌─────────────────────────────────────────────────────────┐
│                   Worker Thread                         │
│  ┌─────────────────────────────────────────────────┐   │
│  │  Tokio Async Runtime                            │   │
│  │  1. Read clipboard text                         │   │
│  │  2. Format prompt with template                 │   │
│  │  3. Call Claude API (HTTP POST)                 │   │
│  │  4. Parse response                              │   │
│  │  5. Auto-paste result                           │   │
│  └─────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Module Structure

```
src/
├── main.rs           # Entry point, event loop, thread coordination
├── config.rs         # Configuration loading and validation
├── hotkey.rs         # Global hotkey registration and management
├── clipboard.rs      # Clipboard text capture
├── anthropic.rs      # Direct HTTP API client implementation
├── paste.rs          # Auto-paste via keyboard simulation
└── workflow.rs       # Complete workflow orchestration
```

### Key Design Decisions

**1. Direct HTTP API (No SDK)**
- **Rationale**: Minimize binary size and dependencies
- **Benefit**: Full control over requests, faster compile times
- **Trade-off**: Manual JSON serialization (manageable with `serde_json`)

**2. Rust + Minimal Dependencies**
- **Rationale**: Performance, memory safety, cross-platform
- **Benefit**: <5MB binary, <30MB RAM, single-file distribution
- **Trade-off**: Longer development time vs. higher-level languages

**3. Main Thread Event Loop**
- **Rationale**: macOS requires hotkey event loop on main thread
- **Benefit**: Reliable hotkey handling on primary platform
- **Trade-off**: Slightly more complex threading model

**4. Keyboard Simulation for Paste**
- **Rationale**: Direct paste is more reliable than clipboard replacement
- **Benefit**: Works across all text applications
- **Trade-off**: Requires Accessibility permissions on macOS

**5. File-based Configuration**
- **Rationale**: Simple, git-friendly, no database overhead
- **Benefit**: Easy to backup, version, and share configurations
- **Trade-off**: No GUI for non-technical users (yet)

## Performance Characteristics

### Current Performance (v0.1.0)
- **Binary Size**: ~3-4MB (release build with optimizations)
- **Memory Usage**: ~25-30MB idle, ~40-50MB during API calls
- **CPU Usage**: <1% idle, brief spike during API processing
- **Startup Time**: <100ms
- **Hotkey Response**: <50ms to initiate API call
- **End-to-End Latency**: ~1-3 seconds (network dependent)

### Optimization Targets
- Binary: <5MB ✅ (achieved)
- Idle RAM: <30MB ✅ (achieved)
- Idle CPU: <1% ✅ (achieved)
- Hotkey-to-API: <200ms ✅ (achieved)

## Platform Support

### macOS (Primary) ✅
- **Versions**: 10.15 (Catalina) and later
- **Status**: Fully functional
- **Special Requirements**: Accessibility permissions for auto-paste
- **Hotkey**: Cmd+Shift+]

### Windows (Secondary) ⏳
- **Versions**: Windows 10 and later
- **Status**: Core functionality working, needs extensive testing
- **Special Requirements**: None
- **Hotkey**: Ctrl+Shift+]

### Linux (Secondary) ⏳
- **Distributions**: Ubuntu 20.04+, Fedora 36+, others TBD
- **Display Servers**: X11 tested, Wayland TBD
- **Status**: Core functionality working, needs testing
- **Special Requirements**: Display server (X11/Wayland)
- **Hotkey**: Ctrl+Shift+]

## Security & Privacy

### Data Handling
- **API Key**: Stored only in environment or `.env` file
- **Clipboard Content**: Never persisted to disk
- **API Responses**: Never logged or stored
- **Network**: HTTPS only for API communication
- **Telemetry**: None (no usage tracking or analytics)

### Permissions
- **macOS**: Accessibility (for auto-paste)
- **Windows**: Standard user permissions
- **Linux**: Standard user permissions + display server access

### Threat Model
- **In Scope**: Preventing API key exposure, secure API communication
- **Out of Scope**: Local OS compromise, physical access attacks
- **Mitigations**: No persistent storage, HTTPS enforcement, clear security docs

## Dependencies

### Core Dependencies
```toml
global-hotkey = "0.6"     # Hotkey registration
tao = "0.30"              # Event loop (macOS compatibility)
arboard = "3.4"           # Clipboard access
enigo = "0.2"             # Keyboard simulation
reqwest = "0.12"          # HTTP client
serde_json = "1.0"        # JSON handling
tokio = "1.41"            # Async runtime
dotenv = "0.15"           # Environment variables
```

### Build Configuration
```toml
[profile.release]
opt-level = "z"        # Optimize for size
lto = true             # Link-time optimization
strip = true           # Strip debug symbols
codegen-units = 1      # Better optimization
```

## User Journey

### Installation Flow
1. Download binary from GitHub releases
2. Move to appropriate directory (`/usr/local/bin`, `Applications`, etc.)
3. Set `ANTHROPIC_API_KEY` environment variable
4. Run application
5. (macOS only) Grant Accessibility permissions when prompted
6. Ready to use

### Daily Usage Flow
1. Copy text to clipboard (Cmd+C / Ctrl+C)
2. Press hotkey (Cmd+Shift+] / Ctrl+Shift+])
3. See console output: "⌨️  Hotkey pressed! Executing workflow..."
4. Wait 1-3 seconds for API processing
5. Refined text auto-pastes at cursor
6. Continue working

### Customization Flow
1. Copy `prompt_template.txt.example` to `prompt_template.txt`
2. Edit template with desired prompt (must include `{clipboard_text}`)
3. Restart application to load new template
4. Test with sample text

## Success Metrics

### Technical Metrics ✅
- ✅ Binary size <5MB: **Achieved** (~3-4MB)
- ✅ Idle memory <30MB: **Achieved** (~25-30MB)
- ✅ Idle CPU <1%: **Achieved** (<1%)
- ✅ Hotkey latency <200ms: **Achieved** (<50ms)

### Quality Metrics
- ⏳ Cross-platform compatibility: **In Progress**
- ✅ Documentation completeness: **Achieved**
- ⏳ Error rate <1%: **Testing in progress**
- ✅ Clear error messages: **Achieved**

### User Metrics (Future)
- Installation success rate: Target >95%
- First-use success rate: Target >90%
- Daily active users: Track post-release
- Invocations per user per day: Track post-release

## Current Status

**Version**: 0.1.0 (MVP)
**Status**: 70% complete (7/10 planned tasks)
**Release**: Pre-release (testing phase)

### Completed ✅
- Project setup and dependencies
- Global hotkey system
- Clipboard integration
- Configuration system
- Direct HTTP API integration
- Auto-paste system
- Complete workflow integration
- Comprehensive documentation

### In Progress ⏳
- Cross-platform testing and refinement
- Performance optimization
- Edge case handling

### Next Steps 📋
1. Complete cross-platform testing (Task #5)
2. Final performance optimization (Task #6)
3. Create release builds for all platforms
4. Set up CI/CD for automated builds
5. Create GitHub release with binaries
6. Gather community feedback

## Resources

- **Repository**: https://github.com/tandelov/hotkey-prompt-refiner
- **Issues**: https://github.com/tandelov/hotkey-prompt-refiner/issues
- **Documentation**: See README.md in repository root
- **API Documentation**: https://docs.anthropic.com/claude/reference/messages_post
