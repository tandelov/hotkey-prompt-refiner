---
name: hotkey-prompt-refiner
status: backlog
created: 2025-10-04T15:47:12Z
progress: 0%
prd: .claude/prds/hotkey-prompt-refiner.md
github: https://github.com/tandelov/hotkey-prompt-refiner/issues/1
---

# Epic: Hotkey Prompt Refiner

## Overview
Build a lightweight, cross-platform Rust desktop application that runs as a background service, allowing users to invoke AI-powered text processing via a global hotkey. The application captures clipboard text, sends it to Claude API with a configurable prompt template, and auto-pastes the response at the cursor location.

## Architecture Decisions

**Language & Runtime:**
- **Rust**: Chosen for minimal resource footprint, cross-platform support, and memory safety
- Single-binary deployment with no runtime dependencies

**Key Technical Choices:**
- **Global Hotkey**: Use `global-hotkey` crate for cross-platform hotkey registration
- **Clipboard**: Use `arboard` crate for clipboard access
- **API Client**: Use `allm` library for Anthropic Claude API integration
- **Auto-paste**: Platform-specific implementations using `enigo` crate for keyboard simulation
- **Config Management**: Use `dotenv` for API key, separate config file for prompt templates
- **Threading Model**: Main thread for macOS hotkey event loop, async runtime for API calls

**Platform Strategy:**
- macOS as primary target with full feature support
- Windows/Linux as secondary targets with core functionality
- Platform-specific code isolated behind trait interfaces

## Technical Approach

### Core Components

**1. Hotkey Manager**
- Register system-wide hotkey (e.g., Cmd+Shift+P)
- Handle platform-specific requirements (macOS main thread constraint)
- Trigger workflow on activation

**2. Clipboard Handler**
- Read text content from system clipboard on hotkey activation
- Validate content is text (skip images, files, etc.)

**3. Prompt Processor**
- Load prompt template from config file
- Inject clipboard text into template
- Format for Claude API

**4. API Client Integration**
- Use `allm` library for Claude API communication
- Handle authentication via environment variable
- Stream or buffer response

**5. Auto-paste System**
- Simulate keyboard input to paste response at cursor
- Platform-specific implementations for reliable cursor targeting
- Fallback to clipboard if paste fails

**6. Configuration System**
- Load API key from `ANTHROPIC_API_KEY` env var or `.env` file
- Read prompt template from external config file (e.g., `prompt_template.txt`)
- Validate configuration on startup

**7. Permission Checker (macOS)**
- Detect Accessibility permissions required for keystroke simulation
- Provide clear error message with instructions if missing

### Infrastructure

**Binary Size & Performance:**
- Use `release` profile with `opt-level = "z"` and `lto = true`
- Strip symbols with `strip = true`
- Target <5MB binary size, <30MB RAM usage

**Error Handling:**
- Clear terminal logging for startup, hotkey registration, and errors
- Graceful degradation if API call fails
- No persistent state or logging of user data

**Security:**
- API key never hardcoded in source
- HTTPS enforced by `allm` library
- No local storage of clipboard content or API responses

## Implementation Strategy

**Phase 1: Core Workflow (macOS)**
1. Set up Rust project with minimal dependencies
2. Implement hotkey registration and clipboard capture
3. Integrate `allm` for Claude API calls
4. Implement auto-paste with keyboard simulation
5. Add configuration loading and validation

**Phase 2: Cross-platform Support**
6. Test and refine Windows compatibility
7. Test and refine Linux compatibility

**Phase 3: Polish & Optimization**
8. Optimize binary size and memory usage
9. Add comprehensive error handling and user feedback
10. Document setup and usage

**Risk Mitigation:**
- Early testing of macOS main thread requirements
- Fallback mechanisms for auto-paste (clipboard as backup)
- Clear permission requirement documentation

**Testing Approach:**
- Manual testing on each platform
- Performance benchmarking (memory, binary size, latency)
- Security review of API key handling

## Task Breakdown Preview

High-level task categories that will be created:
- [ ] **Project Setup**: Initialize Cargo project, configure dependencies, set up development environment
- [ ] **Global Hotkey System**: Implement cross-platform hotkey registration with macOS main thread handling
- [ ] **Clipboard Integration**: Implement clipboard text capture on hotkey activation
- [ ] **API Integration**: Integrate `allm` library, implement prompt templating and API communication
- [ ] **Auto-paste System**: Implement keyboard simulation for response insertion at cursor
- [ ] **Configuration & Security**: Implement config loading, API key management, and macOS permission detection
- [ ] **Cross-platform Testing**: Test and refine Windows/Linux support
- [ ] **Performance Optimization**: Optimize binary size (<5MB) and memory usage (<30MB)
- [ ] **Error Handling & UX**: Add comprehensive logging, error messages, and startup feedback
- [ ] **Documentation**: Create README with setup instructions and usage guide

## Dependencies

**External Services:**
- Anthropic Claude API (requires API key)

**Rust Crates:**
- `global-hotkey`: System-wide hotkey registration
- `arboard`: Clipboard access
- `allm`: Anthropic API client
- `enigo`: Keyboard simulation for auto-paste
- `dotenv`: Environment variable management
- `tokio`: Async runtime for API calls

**Platform Requirements:**
- macOS: Accessibility permissions for keystroke simulation
- Windows: Standard user permissions
- Linux: X11 or Wayland display server

**Prerequisite Work:**
- User must obtain Anthropic API key
- User must configure API key in environment

## Success Criteria (Technical)

**Performance Benchmarks:**
- Binary size: <5MB (release build)
- Idle memory usage: <30MB RAM
- Idle CPU usage: <1%
- Hotkey-to-API-call latency: <200ms

**Quality Gates:**
- Successful hotkey registration and clipboard capture on all platforms
- Reliable API communication with proper error handling
- Accurate auto-paste at cursor location
- No crashes or memory leaks during extended operation
- API key never exposed in logs or error messages

**Acceptance Criteria:**
- User can press hotkey after copying text and receive AI response pasted at cursor
- Application runs as background service with minimal resource usage
- Clear error messages for common issues (missing API key, permissions, etc.)
- Works reliably on macOS, with functional support on Windows/Linux

## Estimated Effort

**Overall Timeline:** 1-2 weeks for MVP (macOS primary), 1 additional week for cross-platform refinement

**Resource Requirements:**
- 1 developer with Rust and systems programming experience
- Access to macOS, Windows, and Linux test environments

**Critical Path Items:**
1. macOS main thread hotkey event loop implementation
2. Reliable auto-paste mechanism across platforms
3. Binary size optimization to meet <5MB target
4. macOS Accessibility permission detection and guidance

## Tasks Created
- [ ] #8 - Project Setup and Dependencies (parallel: false)
- [ ] #9 - Global Hotkey System (parallel: false)
- [ ] #10 - Clipboard Integration (parallel: true)
- [ ] #11 - Configuration System (parallel: true)
- [ ] #2 - API Integration with allm (parallel: false)
- [ ] #3 - Auto-paste System (parallel: true)
- [ ] #4 - Integrate Complete Workflow (parallel: false)
- [ ] #5 - Cross-platform Testing and Refinement (parallel: false)
- [ ] #6 - Performance Optimization (parallel: true)
- [ ] #7 - Documentation and User Guide (parallel: true)

Total tasks: 10
Parallel tasks: 5
Sequential tasks: 5
Estimated total effort: 49-68 hours (~1.5-2 weeks)
