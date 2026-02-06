#!/bin/bash
set -e

# croSShair Installer Script

echo "🔧 Checking dependencies..."
if ! command -v npm &> /dev/null; then
    echo "❌ Error: npm is not installed."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust/Cargo is not installed."
    exit 1
fi

echo "📦 Installing Node dependencies..."
npm install

echo "🔨 Building Release Binary..."
npm run tauri build

echo "✅ Build Complete!"
echo "   Release artifacts are located at:"
echo "   - src-tauri/target/release/crosshair (binary)"

echo ""
read -p "❓ Do you want to install the binary to ~/.local/bin? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    mkdir -p ~/.local/bin
    cp src-tauri/target/release/crosshair ~/.local/bin/
    echo "✅ Installed to ~/.local/bin/crosshair"
    echo "   Ensure ~/.local/bin is in your PATH."
fi
