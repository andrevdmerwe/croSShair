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
    echo "✅ Installed binary to ~/.local/bin/crosshair"

    # Icon and Desktop File
    mkdir -p ~/.local/share/icons/hicolor/128x128/apps/
    cp src-tauri/icons/128x128.png ~/.local/share/icons/hicolor/128x128/apps/crosshair.png
    echo "✅ Installed icon to ~/.local/share/icons/hicolor/128x128/apps/crosshair.png"

    mkdir -p ~/.local/share/applications/
    cat > ~/.local/share/applications/crosshair.desktop <<EOL
[Desktop Entry]
Type=Application
Name=croSShair
Comment=Lightweight screenshot and annotation tool
Exec=$HOME/.local/bin/crosshair
Icon=crosshair
Terminal=false
Categories=Utility;
Keywords=screenshot;capture;annotation;
EOL
    echo "✅ Installed desktop entry to ~/.local/share/applications/crosshair.desktop"
    
    echo "   Ensure ~/.local/bin is in your PATH."
fi
