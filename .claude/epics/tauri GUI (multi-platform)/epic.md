---
name: tauri GUI (multi-platform)
status: backlog
created: 2025-10-05T19:45:53Z
progress: 0%
prd: .claude/prds/tauri GUI (multi-platform).md
github: https://github.com/tandelov/hotkey-prompt-refiner/issues/12
---

# Epic: Tauri GUI (Multi-Platform)

## Overview

Migrate the existing CLI-based Hotkey Prompt Refiner to a Tauri GUI application with Svelte frontend. This transformation will replace manual config file editing with a visual interface while preserving all core hotkey and AI transformation functionality. The implementation leverages existing Rust codebase (hotkey manager, clipboard, API client) and wraps it with Tauri's IPC layer and a lightweight Svelte UI.

**Key Technical Strategy**: Minimal code rewrite by adapting existing Rust modules to work within Tauri's architecture, focusing development effort on UI components and configuration management.

## Architecture Decisions

### 1. Tauri 2.x + Svelte Stack
**Decision**: Use Tauri 2.x with Svelte for frontend
**Rationale**:
- Svelte is the lightest modern framework (smallest bundle size)
- Excellent Tauri integration and documentation
- Reactive stores align well with Tauri's event system
- Single-file components reduce complexity

### 2. Preserve Existing Rust Core
**Decision**: Keep existing `src/anthropic.rs`, `src/clipboard.rs`, hotkey logic
**Rationale**:
- Proven, working code
- Reduce rewrite risk
- Only add Tauri command wrappers around existing functions

### 3. OS Keychain for API Key Storage
**Decision**: Use `keyring-rs` crate for secure API key storage
**Rationale**:
- Native OS credential storage (Keychain on macOS, Secret Service on Linux)
- More secure than encrypted files
- Standard practice for desktop apps

### 4. Single Window + System Tray Architecture
**Decision**: One settings window, menu bar/tray as primary interface
**Rationale**:
- Simpler state management
- Matches user expectations for background utilities
- Reduces memory footprint vs multi-window

### 5. In-Memory History Only
**Decision**: No persistent history storage in v1.0
**Rationale**:
- Privacy-first approach
- Simpler implementation
- Can add persistence later if users request it

## Technical Approach

### Frontend Components (Svelte)

**Core UI Structure**:
```
src-ui/
  ├── App.svelte              # Main app shell, routing
  ├── views/
  │   ├── Settings.svelte     # Settings view (default)
  │   └── History.svelte      # History viewer
  ├── components/
  │   ├── ApiKeyInput.svelte  # API key field + test button
  │   ├── HotkeyRecorder.svelte # "Press key" recorder widget
  │   ├── TemplateEditor.svelte # Template CRUD interface
  │   └── HistoryItem.svelte  # Single history entry
  └── stores/
      ├── settings.js         # Settings state (syncs with backend)
      ├── templates.js        # Template state
      └── history.js          # In-memory history
```

**State Management**:
- Svelte stores for reactive UI state
- Tauri IPC commands for persistence operations
- Event listeners for backend status updates (hotkey triggered, API processing)

### Backend Services (Rust + Tauri)

**Tauri Command Structure**:
```rust
// src/commands.rs (new file)
#[tauri::command]
fn save_api_key(key: String) -> Result<(), String>
#[tauri::command]
fn test_api_key(key: String) -> Result<bool, String>
#[tauri::command]
fn get_templates() -> Result<Vec<Template>, String>
#[tauri::command]
fn save_template(template: Template) -> Result<(), String>
#[tauri::command]
fn delete_template(id: String) -> Result<(), String>
#[tauri::command]
fn set_hotkey(template_id: String, hotkey: Hotkey) -> Result<(), String>
#[tauri::command]
fn get_history() -> Result<Vec<HistoryEntry>, String>
#[tauri::command]
fn clear_history() -> Result<(), String>
```

**Data Models**:
```rust
// src/models.rs (new file)
struct Template {
    id: String,
    name: String,
    description: String,
    prompt: String,
    hotkey: Option<Hotkey>,
}

struct Hotkey {
    modifiers: Vec<String>, // ["cmd", "shift"]
    key: String,            // "]"
}

struct HistoryEntry {
    timestamp: i64,
    template_name: String,
    source_preview: String,   // First 100 chars
    result_preview: String,
    full_source: String,
    full_result: String,
}
```

**Integration Points**:
- Adapt existing `global-hotkey` usage to work with Tauri's event loop
- Expose clipboard operations via Tauri commands
- Emit events when hotkey triggers (`hotkey_triggered`, `api_processing`, `api_complete`)
- Store templates as JSON in config directory
- Use `keyring-rs` for API key CRUD

### Infrastructure

**Build & Distribution**:
- Tauri's built-in bundler for macOS (.dmg, .app)
- AppImage for Linux
- Code signing setup for macOS (future: notarization)

**Configuration Storage**:
- macOS: `~/Library/Application Support/hotkey-prompt-refiner/`
- Linux: `~/.config/hotkey-prompt-refiner/`
- Files: `templates.json`, `settings.json` (non-sensitive settings)

**Auto-Launch**:
- Use `tauri-plugin-autostart` for system boot integration
- Preference stored in `settings.json`

## Implementation Strategy

### Phase 1: Foundation (Week 1)
- Set up Tauri project structure
- Create minimal Svelte UI shell
- Implement Tauri commands for settings CRUD
- Integrate `keyring-rs` for API key storage

### Phase 2: Core Features (Week 2-3)
- Build Settings UI (API key, templates, hotkeys)
- Implement hotkey recorder component
- Adapt existing hotkey system to Tauri event loop
- Wire up clipboard + API integration

### Phase 3: Polish & Platform Support (Week 4)
- Add History viewer
- Implement menu bar/system tray
- macOS-specific testing and permissions handling
- Linux build and testing
- .env migration logic

### Risk Mitigation
- **Hotkey Conflicts**: Test on fresh macOS/Linux installs, provide clear conflict warnings
- **Performance**: Profile early, ensure < 50 MB idle and < 500ms UI load
- **Migration**: Extensive testing with various .env configurations

### Testing Approach
- Unit tests for Tauri commands (Rust)
- Integration tests for hotkey → API → paste flow
- Manual QA on macOS 13+ and Ubuntu 22.04+
- Performance benchmarking vs CLI version

## Task Breakdown Preview

High-level task categories (≤10 tasks):

- [ ] **Task 1**: Tauri project scaffolding and Svelte UI setup
  - Initialize Tauri 2.x project, configure Vite + Svelte
  - Create basic UI shell with routing (Settings/History views)

- [ ] **Task 2**: Backend configuration system and secure storage
  - Implement Tauri commands for settings CRUD
  - Integrate `keyring-rs` for API key storage
  - Build template persistence (JSON files)

- [ ] **Task 3**: Settings UI implementation
  - Build ApiKeyInput component with validation
  - Create TemplateEditor with CRUD operations
  - Implement HotkeyRecorder with "Press Key" interface

- [ ] **Task 4**: Hotkey system integration
  - Adapt existing hotkey manager for Tauri event loop
  - Wire up multi-template hotkey support
  - Implement conflict detection

- [ ] **Task 5**: Clipboard and API integration
  - Expose clipboard operations via Tauri commands
  - Connect existing `src/anthropic.rs` to Tauri
  - Implement event emission for status updates

- [ ] **Task 6**: History viewer and in-memory storage
  - Build History component with search/filter
  - Implement in-memory history store
  - Add copy-to-clipboard from history

- [ ] **Task 7**: Menu bar/system tray integration
  - Implement macOS menu bar with icon
  - Add Linux system tray support
  - Create menu items (Settings, History, Enable/Disable, Quit)

- [ ] **Task 8**: Migration logic and backward compatibility
  - Build .env file detection and import
  - Create migration dialog UI
  - Test with various config scenarios

- [ ] **Task 9**: Auto-launch and platform-specific features
  - Integrate `tauri-plugin-autostart`
  - Handle macOS Accessibility permissions
  - Platform-specific UI polish

- [ ] **Task 10**: Build, test, and distribution setup
  - Configure macOS .dmg bundling
  - Set up Linux AppImage build
  - Performance testing and optimization
  - Create installation documentation

## Dependencies

### External Dependencies
- **Tauri 2.x**: Core framework (stable release required)
- **keyring-rs**: OS credential storage
- **tauri-plugin-autostart**: System boot integration
- **Vite**: Svelte build tooling
- **Anthropic API**: Existing dependency (no changes)

### Internal Dependencies
- Existing Rust codebase must remain functional during migration
- `src/anthropic.rs`: API client (reuse as-is)
- `src/clipboard.rs`: Clipboard handling (adapt for Tauri)
- Hotkey manager: Requires modification for Tauri event loop

### Blocking Risks
- Tauri 2.x global hotkey compatibility on macOS (mitigation: early spike)
- OS keychain availability on Linux (mitigation: fallback to encrypted file)

## Success Criteria (Technical)

### Performance Benchmarks
- ✅ Idle memory: < 50 MB (measure with Activity Monitor/htop)
- ✅ Settings window load: < 500ms (measure time-to-interactive)
- ✅ Hotkey latency: < 200ms press-to-API (same as CLI baseline)
- ✅ Binary size: 3-15 MB (Tauri typical range)

### Quality Gates
- ✅ All Tauri commands have error handling and return Result types
- ✅ API key storage uses OS keychain (verified on macOS + Linux)
- ✅ Hotkey recorder works on first attempt (99% success in testing)
- ✅ Migration successfully imports 10+ different .env configs
- ✅ Zero console errors in Svelte frontend (production build)

### Acceptance Criteria
- ✅ User can complete setup without touching config files
- ✅ Multi-template support with unique hotkeys per template
- ✅ History viewer shows last 50 transformations
- ✅ App launches on system boot (when enabled)
- ✅ macOS and Linux builds tested on 3+ OS versions each

## Estimated Effort

### Overall Timeline
**4 weeks** (solo developer, part-time basis)

- Week 1: Foundation (Tauri setup, basic UI, storage)
- Week 2: Core features (Settings UI, hotkey integration)
- Week 3: Advanced features (History, menu bar, migration)
- Week 4: Testing, polish, build setup

### Resource Requirements
- 1 developer (Rust + Svelte experience)
- Access to macOS and Linux test machines
- Anthropic API key for testing

### Critical Path Items
1. **Tauri hotkey integration** (Week 2) - blocking for core functionality
2. **OS keychain integration** (Week 1) - blocking for security requirements
3. **Migration logic** (Week 3) - blocking for v1.0 release (backward compatibility)

### Simplification Opportunities
- Use existing Rust modules with minimal changes (saves 1+ weeks vs full rewrite)
- Skip Windows support in v1.0 (saves 1 week testing/debugging)
- In-memory history only (saves 3-4 days implementing persistence)
- Single window UI (saves 2-3 days multi-window state management)

**Total Estimated Effort**: ~80-100 hours

## Tasks Created
- [ ] #13 - Migration logic and backward compatibility (parallel: true)
- [ ] #14 - Auto-launch and platform-specific features (parallel: true)
- [ ] #15 - Hotkey system integration (parallel: true)
- [ ] #16 - Build, test, and distribution setup (parallel: false)
- [ ] #17 - Clipboard and API integration (parallel: true)
- [ ] #18 - History viewer and in-memory storage (parallel: false)
- [ ] #19 - Menu bar/system tray integration (parallel: true)
- [ ] #20 - Tauri project scaffolding and Svelte UI setup (parallel: true)
- [ ] #21 - Backend configuration system and secure storage (parallel: true)
- [ ] #22 - Settings UI implementation (parallel: false)

Total tasks: 10
Parallel tasks: 7
Sequential tasks: 3
Estimated total effort: 88-126 hours
