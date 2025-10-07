# Hotkey Prompt Refiner

A lightweight, cross-platform desktop application that enables AI-powered text processing via global hotkeys. Configure custom templates, assign hotkeys, and instantly process clipboard text with Claude AI - all from a clean, modern GUI.

## Features

- **🎛️ GUI Configuration**: Visual settings interface - no config files needed
- **⌨️ Multiple Templates**: Create unlimited prompt templates with unique hotkeys
- **🔐 Secure Storage**: API keys stored in system keychain (macOS Keychain, Linux Secret Service)
- **📋 Clipboard Integration**: Automatically processes text from your clipboard
- **🤖 Claude API**: Direct integration with Anthropic's Claude for intelligent text processing
- **✨ Auto-paste**: AI responses automatically pasted at your cursor location
- **📊 History Viewer**: Review recent transformations with search and export
- **🎯 System Tray**: Quick access via menu bar/system tray icon
- **🚀 Auto-launch**: Optional system startup integration
- **🎨 Modern UI**: Clean Svelte-based interface with dark mode support
- **⚡ Performant**: <50MB RAM usage, <15MB bundle size
- **🌍 Cross-platform**: macOS and Linux support

## Quick Start

### Installation

See [INSTALLATION.md](INSTALLATION.md) for detailed platform-specific instructions.

**macOS:**
1. Download the `.dmg` from [Releases](https://github.com/tandelov/hotkey-prompt-refiner/releases)
2. Drag app to Applications folder
3. Grant Accessibility permissions when prompted

**Linux:**
1. Download the `.AppImage` from [Releases](https://github.com/tandelov/hotkey-prompt-refiner/releases)
2. Make executable: `chmod +x Hotkey-Prompt-Refiner*.AppImage`
3. Run: `./Hotkey-Prompt-Refiner*.AppImage`

### First-Time Setup

1. **Launch the app** - it will appear in your system tray
2. **Open Settings** from the tray menu or window
3. **Add your API key**: Get one at [console.anthropic.com](https://console.anthropic.com/settings/keys)
4. **Create a template**:
   - Click "New Template"
   - Give it a name (e.g., "Grammar Checker")
   - Write your prompt (must include `{clipboard_text}`)
   - Record a hotkey (e.g., Cmd+Shift+G)
   - Save
5. **Test it**:
   - Copy some text
   - Press your hotkey
   - Watch the AI response paste automatically!

## Usage

### Creating Templates

Templates define how Claude processes your text. Each template can have its own hotkey.

**Example templates:**

**Grammar Correction:**
```
Correct the grammar and improve clarity:

{clipboard_text}
```

**Code Review:**
```
Review this code and suggest improvements:

{clipboard_text}

Focus on: performance, security, readability.
```

**Summarization:**
```
Summarize the following text in 2-3 sentences:

{clipboard_text}
```

**Translation:**
```
Translate the following text to Spanish:

{clipboard_text}
```

### Workflow

1. Copy text to clipboard (Cmd+C / Ctrl+C)
2. Press your template's hotkey
3. AI processes the text
4. Result is pasted at cursor location
5. Check History viewer for past transformations

### History

The History view shows your recent transformations:
- Search through past results
- View original text and AI response
- Copy results to clipboard
- Clear history (data is not persisted between sessions)

## Configuration

### API Settings

- **API Key**: Stored securely in system keychain
- **Model Selection**: Choose Claude model (default: claude-3-5-sonnet)
- **Test Connection**: Verify API key before saving

### Templates

- **Name**: Descriptive label for the template
- **Description**: Optional notes about the template's purpose
- **Prompt**: Your instructions to Claude (must include `{clipboard_text}`)
- **Hotkey**: Optional keyboard shortcut (e.g., Cmd+Shift+T)

### System Integration

- **Auto-launch**: Start app on system boot
- **System Tray**: Minimize to tray instead of quitting

## Building from Source

See [BUILDING.md](BUILDING.md) for comprehensive build instructions.

**Quick build:**
```bash
# Clone repository
git clone https://github.com/tandelov/hotkey-prompt-refiner.git
cd hotkey-prompt-refiner

# Install dependencies
npm install

# Run in development
npm run tauri dev

# Build for production
./build-release.sh
```

## Architecture

**Frontend:**
- SvelteKit 2.x with Vite
- Reactive UI with Svelte stores
- Static adapter for Tauri integration

**Backend:**
- Tauri 2.x for native desktop integration
- Rust core with existing hotkey/clipboard modules
- Direct HTTP communication with Claude API (no SDK dependency)

**Storage:**
- API keys: System keychain via `keyring-rs`
- Templates/settings: JSON files in app config directory
- History: In-memory only (privacy-first)

## Troubleshooting

### macOS: Auto-paste not working

Grant Accessibility permissions:
1. System Settings → Privacy & Security → Accessibility
2. Enable "Hotkey Prompt Refiner"
3. Restart the app

### Linux: Hotkeys not working

Ensure your desktop environment supports global hotkeys. Some Wayland compositors have limitations.

### API key not saving

Check permissions for your system keychain:
- **macOS**: Keychain Access should allow the app
- **Linux**: Ensure `gnome-keyring` or equivalent is running

### App won't launch

Check you have required system libraries:
```bash
# Ubuntu/Debian
sudo apt install libwebkit2gtk-4.0-37 libgtk-3-0

# Fedora
sudo dnf install webkit2gtk4.0 gtk3
```

## Privacy & Security

- **API keys**: Stored in OS-native secure storage, never in plaintext
- **No telemetry**: No analytics or tracking
- **No persistence**: History cleared on app exit
- **Local processing**: All API calls go directly to Anthropic (no intermediary)
- **Open source**: Full code available for audit

## Performance

- **Idle RAM**: ~40-50 MB
- **Bundle size**: ~12-15 MB
- **Startup time**: <2 seconds
- **Hotkey latency**: <50ms to API call
- **API response**: Depends on Claude API (~1-3 seconds)

## Requirements

- **macOS**: 10.15+ (Catalina or later)
- **Linux**: Ubuntu 20.04+, Fedora 35+, Arch Linux
- **API**: Anthropic API key (paid)
- **Internet**: Required for Claude API calls

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## License

MIT License - see [LICENSE](LICENSE) for details

## Acknowledgments

Built with:
- [Tauri](https://tauri.app/) - Desktop app framework
- [Svelte](https://svelte.dev/) - Reactive UI framework
- [Claude API](https://www.anthropic.com/claude) - AI text processing

## Support

- **Issues**: [GitHub Issues](https://github.com/tandelov/hotkey-prompt-refiner/issues)
- **Docs**: [Installation Guide](INSTALLATION.md) | [Build Guide](BUILDING.md)
- **API**: [Anthropic Documentation](https://docs.anthropic.com/)

---

**Made with ❤️ for developers who love keyboard-driven workflows**
