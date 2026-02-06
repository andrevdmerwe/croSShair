# croSShair Installation Guide (CachyOS / Arch Linux)

This guide covers how to install croSShair on CachyOS and Arch Linux environments, configure global hotkeys, and update the application.

## Prerequisites

Before building, ensure you have the following installed:

1.  **Rust & Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)
2.  **Node.js & npm**: [Install Node.js](https://nodejs.org/)
3.  **System Dependencies**:
    ```bash
    sudo pacman -S --needed \
        webkit2gtk \
        base-devel \
        curl \
        wget \
        openssl \
        appmenu-gtk-module \
        gtk3 \
        libappindicator-gtk3 \
        librsvg
    ```

## Installation

Since we do not yet distribute pre-built binaries, you will build from source.

### 1. Clone the Repository
```bash
git clone https://github.com/your-username/crosshair.git
cd crosshair
```

### 2. Install Dependencies & Build
You can use the helper script (if available) or run manual steps:

**Option A: Helper Script**
```bash
./scripts/install.sh
```

**Option B: Manual Build**
```bash
# Install Node dependencies
npm install

# Build the release binary
npm run tauri build
```
The built binary/bundle will be located in:
- `src-tauri/target/release/bundle/appimage/` (AppImage)
- `src-tauri/target/release/crosshair` (Raw binary)

### 3. Install
Run the binary directly or copy it to your path:
```bash
cp src-tauri/target/release/crosshair ~/.local/bin/
```

## Global Hotkey Configuration

To take screenshots with `PrintScreen`, you must configure your compositor to launch `croSShair`.

### Hyprland (Common on CachyOS)
Add this to your `~/.config/hypr/hyprland.conf`:
```ini
# Binds PrintScreen to launch croSShair
bind = , Print, exec, crosshair
```

### KDE Plasma (Common on CachyOS)
1. Open **System Settings** > **Shortcuts**.
2. Click **Add New**.
3. Command: `crosshair`.
4. Assign key: `PrintScreen`.

### GNOME
1.  Open **Settings** > **Keyboard** > **View and Customize Shortcuts**.
2.  Go to **Custom Shortcuts**.
3.  Add new:
    - **Name**: croSShair
    - **Command**: `crosshair` (or `/path/to/crosshair`)
    - **Shortcut**: `PrintScreen`.

## Updates

To update to the latest version:

1.  Pull the latest code:
    ```bash
    cd /path/to/crosshair
    git pull
    ```
2.  Rebuild and reinstall:
    ```bash
    ./scripts/install.sh
    ```
