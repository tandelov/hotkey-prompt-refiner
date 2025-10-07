#!/bin/bash

# Hotkey Prompt Refiner - Release Build Script
# Builds production-ready packages for macOS and Linux

set -e  # Exit on error

echo "================================================"
echo "  Hotkey Prompt Refiner - Release Build"
echo "================================================"
echo ""

# Check prerequisites
echo "🔍 Checking prerequisites..."

# Check Node.js
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed"
    exit 1
fi
echo "✓ Node.js $(node --version)"

# Check npm
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed"
    exit 1
fi
echo "✓ npm $(npm --version)"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust/Cargo is not installed"
    exit 1
fi
echo "✓ Cargo $(cargo --version)"

echo ""

# Clean previous builds
echo "🧹 Cleaning previous builds..."
rm -rf src-tauri/target/release/bundle
rm -rf build
rm -rf .svelte-kit
echo "✓ Cleaned build directories"
echo ""

# Install dependencies
echo "📦 Installing dependencies..."
if [ ! -d "node_modules" ]; then
    npm install
else
    echo "✓ Dependencies already installed"
fi
echo ""

# Build frontend
echo "🎨 Building frontend..."
npm run build
echo "✓ Frontend built successfully"
echo ""

# Build Tauri app
echo "🦀 Building Tauri app (this may take a while)..."

# Determine platform
PLATFORM=$(uname -s)
case "${PLATFORM}" in
    Linux*)
        echo "📦 Building for Linux (AppImage + Deb)..."
        npm run tauri build -- --bundles appimage,deb
        BUILD_DIR="src-tauri/target/release/bundle"
        echo ""
        echo "✅ Build complete!"
        echo ""
        echo "📦 Generated packages:"
        echo "  AppImage: $(find ${BUILD_DIR}/appimage -name '*.AppImage' 2>/dev/null | head -1)"
        echo "  Deb:      $(find ${BUILD_DIR}/deb -name '*.deb' 2>/dev/null | head -1)"
        ;;
    Darwin*)
        echo "📦 Building for macOS (DMG + App bundle)..."
        npm run tauri build -- --bundles dmg,app
        BUILD_DIR="src-tauri/target/release/bundle"
        echo ""
        echo "✅ Build complete!"
        echo ""
        echo "📦 Generated packages:"
        echo "  DMG:      $(find ${BUILD_DIR}/dmg -name '*.dmg' 2>/dev/null | head -1)"
        echo "  App:      $(find ${BUILD_DIR}/macos -name '*.app' 2>/dev/null | head -1)"
        ;;
    *)
        echo "❌ Unsupported platform: ${PLATFORM}"
        exit 1
        ;;
esac

echo ""
echo "================================================"
echo "✅ Build completed successfully!"
echo "================================================"
echo ""
echo "📝 Next steps:"
echo "  1. Test the built package"
echo "  2. Sign the package (macOS: codesign, Linux: optional)"
echo "  3. Create a GitHub release"
echo "  4. Upload the packages to the release"
echo ""
