# Building from Source

Complete guide to building **Hotkey Prompt Refiner** from source code.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Clone the Repository](#clone-the-repository)
- [Development Setup](#development-setup)
- [Building for Release](#building-for-release)
- [Platform-Specific Notes](#platform-specific-notes)
- [Troubleshooting Build Issues](#troubleshooting-build-issues)

---

## Prerequisites

### All Platforms

1. **Node.js** 18+ and **npm**
   ```bash
   node --version  # Should be 18.0.0 or higher
   npm --version
   ```
   Download from: https://nodejs.org/

2. **Rust** 1.70+ and **Cargo**
   ```bash
   rustc --version  # Should be 1.70.0 or higher
   cargo --version
   ```
   Install from: https://rustup.rs/

3. **Git**
   ```bash
   git --version
   ```

### macOS Additional Requirements

- **Xcode Command Line Tools**:
  ```bash
  xcode-select --install
  ```

### Linux Additional Requirements

- **Development packages**:

  **Ubuntu/Debian:**
  ```bash
  sudo apt update
  sudo apt install -y \
    libwebkit2gtk-4.0-dev \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    build-essential \
    curl \
    wget \
    file
  ```

  **Fedora:**
  ```bash
  sudo dnf install -y \
    webkit2gtk4.0-devel \
    openssl-devel \
    gtk3-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel
  ```

  **Arch Linux:**
  ```bash
  sudo pacman -S --needed \
    webkit2gtk \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    gtk3 \
    libappindicator-gtk3 \
    librsvg
  ```

---

## Clone the Repository

```bash
git clone https://github.com/yourusername/hotkey-prompt-refiner.git
cd hotkey-prompt-refiner
```

---

## Development Setup

### 1. Install Dependencies

```bash
# Install Node.js dependencies
npm install

# Verify Rust dependencies
cd src-tauri
cargo check
cd ..
```

### 2. Run in Development Mode

```bash
npm run tauri dev
```

This will:
- Start the Vite dev server (frontend)
- Compile and run the Tauri backend
- Enable hot-reload for frontend changes
- Enable fast recompilation for Rust changes

### 3. Development Workflow

**Frontend Development:**
- Edit files in `src/` (Svelte components)
- Changes auto-reload in the app
- Check console for errors: Open DevTools with `Cmd+Option+I` (macOS) or `Ctrl+Shift+I` (Linux)

**Backend Development:**
- Edit files in `src-tauri/src/` (Rust code)
- Save triggers auto-recompilation
- View logs in terminal where you ran `npm run tauri dev`

**Check for errors:**
```bash
# Frontend type checking
npm run check

# Rust compilation check
cd src-tauri && cargo check
```

---

## Building for Release

### Quick Build (Using Script)

```bash
./build-release.sh
```

This script will:
1. Check prerequisites
2. Clean previous builds
3. Install dependencies
4. Build frontend
5. Build Tauri app with optimizations
6. Create platform-specific packages

### Manual Build

#### macOS

```bash
# Build frontend
npm run build

# Build Tauri app (DMG + App bundle)
cd src-tauri
cargo tauri build --bundles dmg,app
cd ..

# Outputs:
# - src-tauri/target/release/bundle/dmg/Hotkey Prompt Refiner_0.1.0_universal.dmg
# - src-tauri/target/release/bundle/macos/Hotkey Prompt Refiner.app
```

#### Linux

```bash
# Build frontend
npm run build

# Build Tauri app (AppImage + Deb)
cd src-tauri
cargo tauri build --bundles appimage,deb
cd ..

# Outputs:
# - src-tauri/target/release/bundle/appimage/hotkey-prompt-refiner_0.1.0_amd64.AppImage
# - src-tauri/target/release/bundle/deb/hotkey-prompt-refiner_0.1.0_amd64.deb
```

### Build Options

**Target specific platforms:**
```bash
cargo tauri build --bundles dmg          # macOS DMG only
cargo tauri build --bundles appimage     # Linux AppImage only
cargo tauri build --bundles deb          # Debian package only
```

**Debug build (faster, larger binary):**
```bash
cargo tauri build --debug
```

**Custom features:**
```bash
cargo tauri build --features "custom-protocol"
```

---

## Platform-Specific Notes

### macOS

#### Code Signing (Optional)

For distribution outside the App Store, you'll want to sign the app:

1. **Get a Developer ID Certificate**:
   - Requires paid Apple Developer account ($99/year)
   - Download from https://developer.apple.com/account/

2. **Sign the build**:
   ```bash
   codesign --deep --force --verify --verbose \
     --sign "Developer ID Application: Your Name (TEAM_ID)" \
     "src-tauri/target/release/bundle/macos/Hotkey Prompt Refiner.app"
   ```

3. **Verify signature**:
   ```bash
   codesign --verify --deep --strict --verbose=2 \
     "src-tauri/target/release/bundle/macos/Hotkey Prompt Refiner.app"
   ```

#### Notarization (Optional)

Required for distribution on macOS 10.15+:

```bash
# Create a DMG first
cargo tauri build --bundles dmg

# Submit for notarization
xcrun notarytool submit \
  "src-tauri/target/release/bundle/dmg/Hotkey Prompt Refiner_0.1.0_universal.dmg" \
  --apple-id "your@email.com" \
  --password "app-specific-password" \
  --team-id "TEAM_ID" \
  --wait

# Staple the notarization ticket
xcrun stapler staple \
  "src-tauri/target/release/bundle/dmg/Hotkey Prompt Refiner_0.1.0_universal.dmg"
```

#### Universal Binary (Apple Silicon + Intel)

The default build creates a universal binary. To build for specific architecture:

```bash
# Apple Silicon only
rustup target add aarch64-apple-darwin
cargo tauri build --target aarch64-apple-darwin

# Intel only
rustup target add x86_64-apple-darwin
cargo tauri build --target x86_64-apple-darwin
```

### Linux

#### AppImage Permissions

AppImages need FUSE to run:

```bash
# Ubuntu/Debian
sudo apt install libfuse2

# Fedora
sudo dnf install fuse fuse-libs

# Arch
sudo pacman -S fuse2
```

#### Building on Different Distributions

The build should work on any Linux distro, but generated packages are native to your build platform:
- **.deb** packages work best on Debian/Ubuntu
- **AppImage** is universal and works everywhere

#### Custom Icons

Place custom icons in `src-tauri/icons/`:
- `icon.png` - 512x512 PNG
- `32x32.png`, `128x128.png`, `128x128@2x.png` - Various sizes

Then rebuild.

---

## Troubleshooting Build Issues

### "Cargo command not found"

Rust is not installed or not in PATH:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### "webkit2gtk not found" (Linux)

Missing system dependencies. Install them:
```bash
# See "Linux Additional Requirements" section above
```

### "No bundle targets specified"

Check `src-tauri/tauri.conf.json`:
```json
{
  "bundle": {
    "active": true,
    "targets": ["dmg", "app", "deb", "appimage"]
  }
}
```

### Build fails with "linking with `cc` failed"

Missing C compiler:
```bash
# macOS
xcode-select --install

# Linux
sudo apt install build-essential  # Debian/Ubuntu
sudo dnf groupinstall "Development Tools"  # Fedora
```

### "openssl-sys" build error

Missing OpenSSL development files:
```bash
# macOS
brew install openssl

# Ubuntu/Debian
sudo apt install libssl-dev

# Fedora
sudo dnf install openssl-devel

# Arch
sudo pacman -S openssl
```

### Frontend build fails

```bash
# Clean and rebuild
rm -rf node_modules package-lock.json
npm install
npm run build
```

### Rust compile takes forever

First build is slow. Subsequent builds are much faster due to incremental compilation.

Speed up builds:
```bash
# Use more CPU cores
export CARGO_BUILD_JOBS=8

# Use faster linker (Linux)
sudo apt install lld
export RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Use faster linker (macOS)
brew install michaeleisel/zld/zld
export RUSTFLAGS="-C link-arg=-fuse-ld=/usr/local/bin/zld"
```

### "Disk full" errors

Release builds can use 2-3 GB of disk space. Clean old builds:
```bash
cd src-tauri
cargo clean
cd ..
rm -rf build .svelte-kit
```

---

## Development Tips

### Fast Iteration

For frontend-only changes:
```bash
npm run dev  # Just frontend, no Tauri
```

For Rust-only changes:
```bash
cd src-tauri
cargo check  # Fast syntax check without full build
cargo clippy  # Linting
```

### Debug Logging

Enable verbose Tauri logs:
```bash
RUST_LOG=debug npm run tauri dev
```

### VSCode Setup

Recommended extensions:
- Svelte for VS Code
- rust-analyzer
- Tauri

`.vscode/settings.json`:
```json
{
  "rust-analyzer.cargo.features": "all",
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

---

## Next Steps

- Read [INSTALLATION.md](INSTALLATION.md) for installation guide
- See [README.md](README.md) for usage documentation
- Check [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines

Happy building! 🛠️
