# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Hotkey Prompt Refiner is a lightweight, cross-platform desktop application (Rust-based) that enables AI-powered text processing via global hotkeys. Users copy text, press a configurable hotkey, and the clipboard content is sent to the Anthropic Claude API with a predetermined prompt. The AI response is automatically pasted at the cursor location.

## Architecture

### Core Components (Planned)

1. **Global Hotkey Manager**: System-wide hotkey registration and handling
   - macOS: Requires main thread event loop and Accessibility permissions
   - Windows/Linux: Standard hotkey registration

2. **Clipboard Handler**: Captures and manages clipboard text content

3. **API Integration**: Direct HTTP communication with Anthropic Claude API using `reqwest` and `serde_json`

4. **Prompt Formatting**: Template-based prompt construction with clipboard text

5. **Auto-paste System**: Injects API responses at the current cursor location

### Platform Considerations

- **macOS (Primary)**: Hotkey event loop must run on main thread; requires Accessibility permissions detection
- **Windows/Linux (Secondary)**: Standard cross-platform hotkey handling

### API Implementation Guidelines

**CRITICAL: DO NOT use any Anthropic API crates (anthropic-sdk, claude-api, etc.)**

This project implements direct HTTP communication with the Anthropic API using ONLY:
- `reqwest` - For HTTP requests
- `serde_json` - For JSON serialization/deserialization

**Rationale:**
- Minimal dependencies = smaller binary size
- Direct control over API requests
- No vendor lock-in to SDK versions
- Meets performance requirements

**API Module:** All Anthropic API communication is handled in `src/anthropic.rs`

**Endpoint:** `https://api.anthropic.com/v1/messages`

**Required Headers:**
- `x-api-key`: API key from environment
- `anthropic-version`: `2023-06-01`
- `content-type`: `application/json`

## Development Commands

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
```

### Test
```bash
cargo test
```

## Configuration

- **API Key**: Set via `ANTHROPIC_API_KEY` environment variable or `.env` file
- **Prompt Template**: Stored in separate config file for easy modifications
- Future: `config.toml` for hotkey bindings, prompt templates, API model selection

## Performance Requirements

- **Memory Usage**: < 30 MB idle
- **Binary Size**: < 5 MB
- **CPU Usage**: < 1% idle
- **Latency**: < 200ms from hotkey press to API call start

## Security Guidelines

- Never store API keys in source code
- All API communication over HTTPS
- No local storage of captured text or responses
