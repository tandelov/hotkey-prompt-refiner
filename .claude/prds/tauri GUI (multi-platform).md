---
name: tauri GUI (multi-platform)
description: Migrate CLI-based hotkey prompt refiner to Tauri GUI with visual configuration, menu bar integration, and multi-platform support
status: backlog
created: 2025-10-05T19:32:52Z
---

# PRD: Tauri GUI (Multi-Platform)

## Executive Summary

Transform the Hotkey Prompt Refiner from a CLI-based tool into a user-friendly GUI application using Tauri. The GUI will provide visual configuration for API keys, hotkeys, and prompt templates, making the tool accessible to non-technical users while maintaining the core functionality of AI-powered text transformation via global hotkeys.

**Key Value Proposition**: Eliminate the need for manual config file editing by providing an intuitive visual interface for all settings, hotkey configuration, and prompt management.

## Problem Statement

### Current Pain Points
- **Configuration Complexity**: Users must manually edit `.env` files and config files to set API keys, change hotkeys, or modify prompt templates
- **Hotkey Setup**: No visual hotkey recorder - users must know key codes or reference documentation
- **Template Management**: Switching between multiple prompt templates requires file system navigation and editing
- **No Visibility**: CLI provides no visual feedback on status, history, or recent transformations
- **Non-Technical Barrier**: Current implementation requires technical knowledge, limiting adoption

### Why Now?
The core hotkey functionality is proven and working. Adding a GUI will dramatically improve user experience and expand the user base to include non-technical users who need AI-powered text transformation without CLI complexity.

## User Stories

### Primary Persona: Non-Technical Knowledge Worker
**Sarah, Marketing Manager**
- Needs to refine copy and generate content variations quickly
- Comfortable with desktop apps, not with config files
- Wants to switch between different prompt templates for different use cases
- Values visual feedback and ease of setup

### User Journeys

#### First-Time Setup
1. Sarah downloads and installs the app
2. App starts automatically in menu bar (macOS) or system tray (Windows/Linux)
3. She clicks the menu bar icon and selects "Settings"
4. Settings window opens with clear input fields for API key
5. She enters her Anthropic API key and saves
6. She configures her first hotkey using a "Press Key" recorder interface
7. She edits the default prompt template or creates a new one
8. Setup complete - ready to use

#### Daily Usage
1. Sarah copies text she wants to refine
2. She presses her configured hotkey (e.g., Cmd+Shift+])
3. Menu bar icon shows brief activity indicator
4. AI-refined text automatically pastes at cursor location
5. She clicks menu bar → "History" to review recent transformations if needed

#### Template Switching
1. Sarah needs different prompts for different tasks (grammar check vs. creative rewrite)
2. She opens Settings → Prompt Templates
3. She creates new templates with different prompts and assigns different hotkeys
4. She can now use multiple hotkeys for different AI transformations

## Requirements

### Functional Requirements

#### FR1: Settings/Configuration UI
- Visual editor for Anthropic API key (password-masked input)
- API key validation with connection test button
- Support for .env file compatibility during migration period
- Settings persistence in secure local storage

#### FR2: Hotkey Configuration
- Visual hotkey recorder with "Press Key" interface
- Support for modifier keys (Cmd/Ctrl, Shift, Alt/Option)
- Real-time conflict detection (warn if hotkey already in use)
- Cross-platform hotkey mapping (macOS, Linux, Windows)
- Multiple hotkey profiles for different prompt templates

#### FR3: Prompt Template Management
- Create, edit, delete prompt templates
- Template list view with names and descriptions
- Rich text editor for prompt content
- Template variables support (e.g., {clipboard_text})
- Set active/default template
- Assign unique hotkey to each template
- Export/import templates (JSON format)

#### FR4: Menu Bar/System Tray Integration
- macOS: Native menu bar icon with dropdown menu
- Linux/Windows: System tray icon with context menu
- Menu items:
  - Settings
  - History
  - Enable/Disable (toggle active state)
  - Quit
- Visual status indicator (active/inactive/processing)

#### FR5: History & Activity Log
- Display recent transformations (last 50-100 entries)
- Show: timestamp, source text preview, result preview, template used
- Search/filter history
- Clear history option
- Copy source or result text from history
- No persistent storage (privacy-focused, session-based only)

#### FR6: Auto-Launch on System Boot
- Enable/disable auto-start setting
- Run in background by default
- Minimize to menu bar/tray on startup

### Non-Functional Requirements

#### NFR1: Performance
- Idle memory usage: < 50 MB (relaxed from CLI's 30 MB for GUI overhead)
- Binary size: 3-15 MB (Tauri typical range)
- Idle CPU usage: < 2%
- Hotkey response latency: < 200ms from press to API call start
- Settings window load time: < 500ms

#### NFR2: Security
- API keys stored in OS-native secure storage (Keychain on macOS, Credential Manager on Windows)
- Fallback to encrypted local storage with OS-provided encryption
- No plaintext API keys in config files (post-migration)
- All API communication over HTTPS
- No cloud sync of sensitive data

#### NFR3: Platform Support
- **Phase 1 (v1.0)**: macOS (primary focus)
- **Phase 1 (v1.0)**: Linux (required)
- **Phase 2 (v1.1+)**: Windows (later)
- Native look and feel on each platform
- Platform-specific menu bar/tray implementations

#### NFR4: Usability
- Settings window accessible within 1 click from menu bar
- No technical jargon in UI labels
- Inline help text for key settings
- Error messages with actionable guidance
- Intuitive visual hierarchy and layout

#### NFR5: Reliability
- Graceful API error handling with user-friendly messages
- Automatic retry logic for transient network failures
- Settings validation before save
- Crash recovery (restore last known good config)

## Success Criteria

### Launch Metrics (3 months post-v1.0)
- **User Adoption**: 80% of existing CLI users migrate to GUI version
- **Setup Completion**: 95% of new users complete first-time setup without support
- **Active Usage**: Average 10+ hotkey invocations per user per day
- **Template Usage**: 60% of users create 2+ custom templates

### Performance Metrics
- Zero-latency perception: 90% of hotkey presses feel instant (< 200ms to API)
- Settings load time: 100% under 500ms
- Memory footprint: 95% of sessions stay under 50 MB idle

### Quality Metrics
- Bug-free hotkey capture: 99% success rate on first attempt
- API key validation accuracy: 100% correct detection of valid/invalid keys
- Cross-platform parity: Feature parity between macOS and Linux in v1.0

## Technical Architecture

### Technology Stack
- **Backend**: Rust (existing codebase)
- **GUI Framework**: Tauri 2.x
- **Frontend**: Svelte (lightweight, modern, excellent Tauri integration)
- **State Management**: Svelte stores + Tauri state management
- **Secure Storage**: keyring-rs (cross-platform OS credential storage)

### Key Components
1. **Tauri Backend (Rust)**
   - Existing hotkey manager, clipboard handler, API client
   - Tauri commands for settings CRUD
   - Secure storage integration
   - System tray/menu bar management

2. **Frontend (Svelte)**
   - Settings UI components
   - Hotkey recorder component
   - Template editor
   - History viewer
   - Responsive layout (single window, multiple views)

3. **IPC Bridge**
   - Tauri's invoke/emit for frontend-backend communication
   - Event system for status updates
   - State synchronization

### Data Storage
- **API Keys**: OS keychain/credential manager
- **App Settings**: JSON in OS-appropriate config directory
  - macOS: `~/Library/Application Support/hotkey-prompt-refiner/`
  - Linux: `~/.config/hotkey-prompt-refiner/`
  - Windows: `%APPDATA%/hotkey-prompt-refiner/`
- **Templates**: JSON files in config directory
- **History**: In-memory only (privacy-focused)

## Constraints & Assumptions

### Technical Constraints
- Tauri webview availability on target platforms
- macOS: Requires Accessibility permissions (existing constraint)
- Global hotkey registration limits (OS-dependent)
- Webview rendering performance for UI responsiveness

### Design Constraints
- Single window application (no multi-window complexity)
- Menu bar/tray as primary interaction point
- Settings window is modal-like (one instance at a time)

### Resource Constraints
- Solo developer project (phased rollout: macOS → Linux → Windows)
- No cloud infrastructure (local-only application)

### Assumptions
- Users have Anthropic API keys or can obtain them
- Target platforms support global hotkey registration
- OS secure storage is available and reliable
- Existing .env configs will be migrated automatically on first launch

## Out of Scope

### Explicitly NOT Building (v1.0)
- ❌ Built-in prompt testing interface (can test by copying text and using hotkey)
- ❌ Toast/push notifications for success/failure
- ❌ Multi-user support or user profiles
- ❌ Cloud sync of settings/templates
- ❌ Template marketplace or sharing
- ❌ Advanced prompt engineering tools (variables, conditionals, loops)
- ❌ Support for other AI providers (Claude API only)
- ❌ Mobile versions (desktop only)
- ❌ Offline mode or local LLM support
- ❌ Windows platform (v1.0 - deferred to v1.1+)

### Future Considerations (Post-v1.0)
- Windows support (v1.1)
- Template import from community library
- Keyboard shortcut customization beyond hotkeys (app-level shortcuts)
- Advanced history features (export, analytics)
- Plugin system for custom transformations

## Dependencies

### External Dependencies
- **Anthropic API**: Claude API availability and rate limits
- **Tauri Framework**: Tauri 2.x stability and platform support
- **OS APIs**:
  - Accessibility APIs (macOS)
  - Global hotkey registration (all platforms)
  - Secure credential storage (all platforms)
  - System tray/menu bar APIs (all platforms)

### Internal Dependencies
- Existing Rust codebase (hotkey manager, clipboard, API client)
- Migration from `global-hotkey` to Tauri-compatible hotkey system
- Tauri-specific clipboard integration

### Timeline Dependencies
- Phase 1: macOS + Linux implementation (v1.0)
- Phase 2: Windows support (v1.1+) - blocks full multi-platform claim

### Build Dependencies
- Tauri CLI tooling
- Svelte build tools (Vite)
- Platform-specific signing certificates (for macOS distribution)

## Migration Strategy

### CLI → GUI Transition

#### Backward Compatibility (v1.0)
- Detect and import existing `.env` file on first launch
- Parse existing config files if present
- Migrate settings to new GUI storage format
- Keep .env as fallback during transition period

#### Migration UX
1. First launch detects existing CLI config
2. Show migration dialog: "Import existing settings?"
3. Auto-import API key, hotkeys, and templates
4. Confirm successful migration
5. Offer to keep or remove old config files

#### Deprecation Timeline
- v1.0: Full backward compatibility
- v1.1: Warning for .env usage (recommend GUI settings)
- v2.0: Remove .env support entirely

## Open Questions

1. **Template Variables**: What level of template variable support is needed beyond {clipboard_text}? (e.g., {date}, {selected_template_name})
2. **History Persistence**: Should users have option to enable persistent history storage despite privacy focus?
3. **Hotkey Conflicts**: How aggressive should conflict detection be? (warn only, or prevent save?)
4. **Update Mechanism**: How will users receive updates? (auto-update, manual download, app store?)
5. **Telemetry**: Any anonymous usage metrics for improving UX? (opt-in only)

## Risks & Mitigation

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| Tauri bundle size exceeds expectations | Medium | Low | Monitor build size, optimize assets, consider code splitting |
| Platform-specific hotkey bugs | High | Medium | Thorough testing on each platform, fallback mechanisms |
| OS secure storage unavailable | High | Low | Fallback to encrypted local storage with strong warnings |
| Migration from CLI fails | Medium | Medium | Extensive testing, clear error messages, manual import option |
| Performance degradation vs CLI | Medium | Low | Profiling, optimization, stay within NFR targets |

## Success Validation

### Pre-Launch Checklist
- [ ] All FR requirements implemented and tested
- [ ] Performance metrics meet NFR targets
- [ ] macOS and Linux builds tested on multiple OS versions
- [ ] Migration from existing CLI configs tested with real user data
- [ ] Security audit of API key storage and transmission
- [ ] Accessibility permissions flow tested on macOS

### Post-Launch Validation
- [ ] User feedback collected via GitHub issues
- [ ] Performance monitoring via crash reports
- [ ] Migration success rate tracked (successful imports vs. manual setups)
- [ ] User retention tracked (continued usage after 1 week, 1 month)

---

## Next Steps

1. **Technical Spike**: Evaluate Tauri + Svelte integration, build proof-of-concept settings UI
2. **Design Mockups**: Create UI wireframes for settings window, menu bar, history view
3. **Epic Creation**: Break down implementation into development epics and stories
4. **Platform Testing**: Set up test environments for macOS and Linux

**Ready to proceed?** Run: `/pm:prd-parse tauri GUI (multi-platform)` to create implementation epic.
