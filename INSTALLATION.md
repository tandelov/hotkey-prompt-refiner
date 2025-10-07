# Installation Guide

Complete installation instructions for **Hotkey Prompt Refiner** on macOS and Linux.

## Table of Contents

- [System Requirements](#system-requirements)
- [macOS Installation](#macos-installation)
- [Linux Installation](#linux-installation)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)
- [Uninstallation](#uninstallation)

---

## System Requirements

### macOS
- macOS 10.15 (Catalina) or later
- Apple Silicon (M1/M2/M3) or Intel processor
- 50 MB free disk space
- Internet connection for API calls

### Linux
- Ubuntu 20.04+ / Debian 11+ / Fedora 35+ / Arch Linux
- X11 or Wayland display server
- 50 MB free disk space
- Internet connection for API calls

### Both Platforms
- Anthropic API key (get one at [console.anthropic.com](https://console.anthropic.com/settings/keys))

---

## macOS Installation

### Option 1: DMG Installer (Recommended)

1. **Download** the latest `.dmg` file from the [Releases page](https://github.com/yourusername/hotkey-prompt-refiner/releases)

2. **Open** the downloaded DMG file

3. **Drag** the "Hotkey Prompt Refiner" app to your Applications folder

4. **Launch** the app from Applications

5. **Grant Permissions** (First launch only):
   - macOS will show "App is from an unidentified developer"
   - Go to **System Settings** → **Privacy & Security**
   - Click **"Open Anyway"** next to the Hotkey Prompt Refiner message
   - Click **"Open"** in the confirmation dialog

6. **Accessibility Permission** (Required for auto-paste):
   - The app will prompt you to grant Accessibility permission
   - Go to **System Settings** → **Privacy & Security** → **Accessibility**
   - Click the lock icon and authenticate
   - Enable "Hotkey Prompt Refiner"
   - Restart the app

### Option 2: App Bundle

If you downloaded the `.app` bundle directly:

1. Move the app to `/Applications/`
2. Follow steps 5-6 above

### macOS Code Signing (Developer)

For distribution, sign the app:

```bash
# Sign the app
codesign --deep --force --verify --verbose --sign "Developer ID Application: Your Name" \
  "Hotkey Prompt Refiner.app"

# Notarize (requires Apple Developer account)
xcrun notarytool submit "Hotkey Prompt Refiner.dmg" \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait

# Staple notarization
xcrun stapler staple "Hotkey Prompt Refiner.dmg"
```

---

## Linux Installation

### Option 1: AppImage (Universal)

AppImage works on most Linux distributions without installation.

1. **Download** the latest `.AppImage` file from the [Releases page](https://github.com/yourusername/hotkey-prompt-refiner/releases)

2. **Make executable**:
   ```bash
   chmod +x Hotkey-Prompt-Refiner_0.1.0_amd64.AppImage
   ```

3. **Run**:
   ```bash
   ./Hotkey-Prompt-Refiner_0.1.0_amd64.AppImage
   ```

4. **Optional: Integrate with system** (using AppImageLauncher):
   ```bash
   # Install AppImageLauncher (Ubuntu/Debian)
   sudo add-apt-repository ppa:appimagelauncher-team/stable
   sudo apt update
   sudo apt install appimagelauncher

   # Then double-click the AppImage - it will integrate automatically
   ```

### Option 2: Debian Package (Ubuntu/Debian)

1. **Download** the `.deb` file

2. **Install**:
   ```bash
   sudo dpkg -i hotkey-prompt-refiner_0.1.0_amd64.deb

   # Fix dependencies if needed
   sudo apt-get install -f
   ```

3. **Launch**:
   ```bash
   hotkey-prompt-refiner
   ```
   Or find it in your application menu.

### Option 3: Build from Source

See [BUILDING.md](BUILDING.md) for detailed build instructions.

### Linux Permissions

The app requires X11 input simulation permissions. Most distributions grant this by default, but if you encounter issues:

```bash
# Check X11 access
xhost +local:

# For Wayland, ensure XWayland compatibility:
sudo apt install xwayland  # Ubuntu/Debian
```

---

## Configuration

### First Launch

1. **Launch** the application
2. The **Settings** page will open automatically
3. **Enter your Anthropic API key**
4. Click **"Save"** and then **"Test"** to verify

### Creating Your First Template

1. Go to **Settings** → **Prompt Templates**
2. Click **"+ Add Template"**
3. Fill in:
   - **Name**: e.g., "Fix Grammar"
   - **Description**: Optional description
   - **Prompt Template**: Your prompt with `{clipboard_text}` placeholder
     ```
     Fix all grammar and spelling errors in the following text:

     {clipboard_text}
     ```
   - **Hotkey**: e.g., `Cmd+Shift+G` (macOS) or `Ctrl+Shift+G` (Linux)
4. Click **"Create"**

### Setting Up Auto-Launch

1. Go to **Settings** → **Application Settings**
2. Toggle **"Launch at Login"** on
3. The app will now start automatically when you log in

---

## Troubleshooting

### macOS Issues

#### "App cannot be opened because the developer cannot be verified"
- Go to **System Settings** → **Privacy & Security**
- Click **"Open Anyway"** next to the app name

#### Hotkeys not working
- Grant **Accessibility** permission:
  - **System Settings** → **Privacy & Security** → **Accessibility**
  - Add and enable "Hotkey Prompt Refiner"
  - Restart the app

#### Auto-paste not working
- Same as above - Accessibility permission is required

#### System tray icon not showing
- Check if the app is running: look for it in Activity Monitor
- Try quitting and relaunching

### Linux Issues

#### AppImage won't run
- Ensure it's executable: `chmod +x Hotkey-Prompt-Refiner*.AppImage`
- Install FUSE: `sudo apt install libfuse2` (Ubuntu/Debian)

#### Hotkeys not working
- Check X11/Wayland compatibility
- Verify no other app is using the same hotkey
- Try a different hotkey combination

#### System tray icon not showing
- Install a system tray extension:
  - **GNOME**: Install "AppIndicator" extension
  - **KDE**: Should work by default
  - **i3/others**: Install `trayer` or similar

#### Permission denied errors
```bash
# Grant execution permissions
sudo chmod +x /path/to/app

# For AppImage
sudo chmod 755 Hotkey-Prompt-Refiner*.AppImage
```

### Common Issues (All Platforms)

#### "No API key configured"
- Go to **Settings** and enter your Anthropic API key
- Get one from: https://console.anthropic.com/settings/keys

#### "Failed to read clipboard"
- Copy text before pressing the hotkey
- Check clipboard access permissions

#### "Hotkey is already in use"
- Another application is using the same hotkey
- Choose a different hotkey combination
- Close conflicting applications

#### API calls failing
- Check your internet connection
- Verify your API key is valid (test in Settings)
- Check Anthropic API status: https://status.anthropic.com

---

## Uninstallation

### macOS

1. **Quit** the app (right-click tray icon → Quit)
2. **Delete** `/Applications/Hotkey Prompt Refiner.app`
3. **Remove settings** (optional):
   ```bash
   rm -rf ~/Library/Application\ Support/com.hotkey-prompt-refiner.app
   rm -rf ~/Library/Preferences/com.hotkey-prompt-refiner.app.plist
   ```
4. **Remove from Keychain** (optional):
   - Open **Keychain Access**
   - Search for "hotkey-prompt-refiner"
   - Delete any found entries

### Linux

#### AppImage
Simply delete the `.AppImage` file

#### Debian Package
```bash
sudo apt remove hotkey-prompt-refiner
```

#### Remove settings (optional)
```bash
rm -rf ~/.config/hotkey-prompt-refiner
rm -rf ~/.local/share/hotkey-prompt-refiner
```

---

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/yourusername/hotkey-prompt-refiner/issues)
- **Documentation**: [README.md](README.md)
- **Build Guide**: [BUILDING.md](BUILDING.md)

---

## Next Steps

- Configure your templates in **Settings**
- Set up **auto-launch** for convenience
- View your processing **History**
- Customize hotkeys for different tasks

Enjoy using Hotkey Prompt Refiner! 🚀
