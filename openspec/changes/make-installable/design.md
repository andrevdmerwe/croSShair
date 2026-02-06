# Design: Installation Support

## Architecture

No core architectural changes to the application logic. The changes are focused on:
1.  **Configuration**: Updating `tauri.conf.json` for correct bundling.
2.  **Documentation**: Adding `install_step.md` to the package root.
3.  **Automation**: (Optional) Adding `scripts/install.sh`.

## Data Model

No database or state changes.

## Interface

### CLI / Scripts
- `npm run tauri build`: Standard Tauri build command.
- `scripts/install.sh` (Proposed):
    ```bash
    #!/bin/bash
    # Installs dependencies, builds, and copies binary
    ```

### Documentation (`install_step.md`)
Structure:
- **Introduction**: Brief overview.
- **Prerequisites**: List of packages (libwebkit2gtk-4.0-dev, etc.).
- **Installation**:
    1. Clone repo.
    2. Run install script OR manual build steps.
- **Global Hotkey Setup**:
    - **GNOME**: Settings -> Keyboard -> Shortcuts.
    - **Hyprland**: `bind = , Print, exec, croSShair`.
    - **Sway/i3**: `bindsym Print exec croSShair`.
- **Updates**: `git pull && npm run tauri build`.

## Migration Strategy

- Update `tauri.conf.json` immediately.
- Create `install_step.md`.
- Verify build process produces working deb/appimage.
