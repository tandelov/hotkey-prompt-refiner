# Hotkey Prompt Refiner

A lightweight, cross-platform desktop application that enables AI-powered text processing via global hotkeys. Copy text, press a hotkey, and instantly receive an AI-refined response pasted at your cursor location.

## Features

- **Global Hotkey Activation**: Trigger AI processing from anywhere with `Cmd+Shift+]` (macOS) or `Ctrl+Shift+]` (Windows/Linux)
- **Clipboard Integration**: Automatically processes text from your clipboard
- **Claude API Integration**: Direct HTTP integration with Anthropic's Claude API for intelligent text processing
- **Auto-paste**: AI responses are automatically pasted at your cursor location
- **Customizable Prompts**: Configure prompt templates to tailor AI processing to your needs
- **Minimal Resource Usage**: <5MB binary size, <30MB RAM usage, <1% idle CPU
- **Cross-platform**: Works on macOS, Windows, and Linux

## Installation

### Prerequisites

- Anthropic API key (get one at https://console.anthropic.com/settings/keys)

### macOS

1. Download the latest release from [Releases](https://github.com/tandelov/hotkey-prompt-refiner/releases)
2. Extract and move the binary to a convenient location (e.g., `/usr/local/bin/`)
3. Make it executable: `chmod +x hotkey-prompt-refiner`
4. **Important**: Grant Accessibility permissions when prompted (required for auto-paste functionality)
   - Go to System Preferences → Security & Privacy → Privacy → Accessibility
   - Add and enable the application

### Windows

1. Download the latest release from [Releases](https://github.com/tandelov/hotkey-prompt-refiner/releases)
2. Extract the binary to a convenient location
3. Run `hotkey-prompt-refiner.exe`

### Linux

1. Download the latest release from [Releases](https://github.com/tandelov/hotkey-prompt-refiner/releases)
2. Extract and move the binary to `/usr/local/bin/` or another location in your PATH
3. Make it executable: `chmod +x hotkey-prompt-refiner`
4. Ensure X11 or Wayland display server is running

### Building from Source

```bash
# Clone the repository
git clone https://github.com/tandelov/hotkey-prompt-refiner.git
cd hotkey-prompt-refiner

# Build release binary
cargo build --release

# Binary will be at target/release/hotkey-prompt-refiner
```

## Configuration

### API Key Setup

Set your Anthropic API key using one of these methods:

**Option 1: Environment Variable**
```bash
export ANTHROPIC_API_KEY="sk-ant-..."
```

**Option 2: .env File**
Create a `.env` file in the same directory as the binary:
```
ANTHROPIC_API_KEY=sk-ant-...
```

### Customizing the Prompt Template (Optional)

The default prompt refines and improves text. To customize:

1. Copy the example template:
   ```bash
   cp prompt_template.txt.example prompt_template.txt
   ```

2. Edit `prompt_template.txt` with your desired prompt. **Must include `{clipboard_text}` placeholder**

Example templates:

**Code Review:**
```
Review the following code and provide suggestions for improvements:

{clipboard_text}

Focus on: code quality, performance, security, and best practices.
```

**Grammar Correction:**
```
Correct the grammar and improve the clarity of the following text:

{clipboard_text}

Maintain the original tone and meaning.
```

**Summarization:**
```
Summarize the following text in 2-3 concise bullet points:

{clipboard_text}
```

## Usage

1. **Start the application**:
   ```bash
   ./hotkey-prompt-refiner
   ```

   You should see:
   ```
   ✓ Configuration loaded successfully
   ✓ Configuration validated
   ✓ Hotkey manager initialized
   ✓ Hotkey registered: Cmd+Shift+]

   ✅ System ready! Press Cmd+Shift+] to process clipboard text.
   ```

2. **Copy text** to your clipboard (e.g., select text and press `Cmd+C`)

3. **Press the hotkey**:
   - macOS: `Cmd+Shift+]`
   - Windows/Linux: `Ctrl+Shift+]`

4. **Wait for processing**: The app sends your text to Claude API

5. **AI response auto-pastes** at your cursor location

### Example Workflow

```
1. Copy: "make this better: the app dont work good"
2. Press: Cmd+Shift+]
3. Get pasted: "The application isn't functioning properly"
```

## Troubleshooting

### "ANTHROPIC_API_KEY not found in environment or .env file"

**Solution**: Set your API key using one of the methods in [Configuration](#api-key-setup)

### "Failed to register hotkey" or "Hotkey is already in use"

**Cause**: Another application is using `Cmd+Shift+]` or `Ctrl+Shift+]`

**Solution**: Close conflicting applications or modify the source code to use a different hotkey in `src/hotkey.rs`

### macOS: "Accessibility permissions required"

**Cause**: macOS requires Accessibility permissions for the auto-paste feature to simulate keystrokes

**Solution**:
1. Go to System Preferences → Security & Privacy → Privacy → Accessibility
2. Click the lock icon to make changes
3. Add the application to the list
4. Enable the checkbox next to it
5. Restart the application

### "API error: 401 Unauthorized"

**Cause**: Invalid or expired API key

**Solution**: Verify your API key at https://console.anthropic.com/settings/keys and update your configuration

### "API error: 400 Bad Request"

**Cause**: Prompt template validation failed

**Solution**: Ensure your `prompt_template.txt` contains the `{clipboard_text}` placeholder

### Response not pasting

**Cause**:
- Missing Accessibility permissions (macOS)
- Application doesn't have focus on active text field

**Solution**:
- Grant Accessibility permissions (macOS)
- Ensure cursor is in a text input field before pressing hotkey
- Check console output for error messages

### High memory usage

**Expected**: <30MB idle, may increase during API calls

**If excessive**: Check for memory leaks and report an issue with logs

## Performance

- **Binary Size**: ~3-5MB (release build)
- **Memory Usage**: <30MB idle, ~40-50MB during API processing
- **CPU Usage**: <1% idle
- **Response Latency**: <200ms from hotkey press to API call (network time depends on API)

## Platform-Specific Notes

### macOS
- Requires Accessibility permissions for auto-paste
- Hotkey event loop runs on main thread (required by macOS)
- Tested on macOS 10.15+

### Windows
- Standard user permissions sufficient
- Tested on Windows 10+

### Linux
- Requires X11 or Wayland display server
- Some window managers may require additional configuration for global hotkeys
- Tested on Ubuntu 20.04+ and Fedora 36+

## Development

### Project Structure

```
hotkey-prompt-refiner/
├── src/
│   ├── main.rs           # Application entry point and event loop
│   ├── anthropic.rs      # Direct HTTP API client (reqwest + serde_json)
│   ├── clipboard.rs      # Clipboard text capture
│   ├── config.rs         # Configuration management
│   ├── hotkey.rs         # Global hotkey registration
│   ├── paste.rs          # Auto-paste implementation
│   └── workflow.rs       # Complete workflow orchestration
├── Cargo.toml            # Dependencies and build configuration
├── .env.example          # Example environment variables
└── prompt_template.txt.example  # Example prompt template
```

### Architecture

**Threading Model:**
- Main thread: Runs hotkey event loop (required for macOS)
- Worker thread: Handles async API calls using Tokio runtime

**API Implementation:**
- Uses direct HTTP calls via `reqwest` (no SDK dependencies)
- JSON serialization with `serde_json`
- Endpoint: `https://api.anthropic.com/v1/messages`
- Model: `claude-3-5-haiku-20241022` (configurable in code)

### Dependencies

Core dependencies (minimal by design):
- `global-hotkey`: System-wide hotkey registration
- `tao`: Event loop for macOS compatibility
- `arboard`: Cross-platform clipboard access
- `enigo`: Keyboard simulation for auto-paste
- `reqwest`: HTTP client for API calls
- `serde_json`: JSON handling
- `tokio`: Async runtime
- `dotenv`: Environment variable management

### Building

```bash
# Debug build
cargo build

# Release build (optimized for size)
cargo build --release

# Run tests
cargo test

# Check code
cargo clippy
```

### Build Optimizations

The release profile is optimized for binary size:
```toml
[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
strip = true          # Strip symbols
codegen-units = 1     # Better optimization
```

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/your-feature`
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

**Contribution Guidelines:**
- Follow Rust idioms and best practices
- Maintain minimal dependencies
- Keep binary size and memory usage low
- Test on multiple platforms when possible
- Update documentation for new features

## Known Limitations

- Hotkey cannot be changed without recompiling (planned feature)
- Single prompt template at a time (planned: multiple templates)
- No GUI for configuration (CLI-only by design for minimal footprint)
- macOS Accessibility permissions required for auto-paste

## Roadmap

- [ ] Configurable hotkeys via config file
- [ ] Multiple prompt templates with template selection
- [ ] Streaming API responses for faster perceived performance
- [ ] Hotkey for clipboard-only mode (no paste)
- [ ] Menu bar integration (macOS)
- [ ] System tray integration (Windows/Linux)

## Security

- API key stored only in environment variables or `.env` file (never in source code)
- All API communication over HTTPS
- No persistent storage of clipboard content or API responses
- No telemetry or analytics

## License

MIT License - See LICENSE file for details

## Support

- **Issues**: https://github.com/tandelov/hotkey-prompt-refiner/issues
- **Discussions**: https://github.com/tandelov/hotkey-prompt-refiner/discussions

## Acknowledgments

Built with:
- [Anthropic Claude API](https://www.anthropic.com/api)
- Rust and the amazing crates ecosystem
