# Capability Spec: Installation and Configuration

## Requirements

### Local Installation
- **REQ-INSTALL-001**: Users MUST be able to install the application locally using standard Rust/Tauri tools.
- **REQ-INSTALL-002**: Configuration for Linux bundling (deb, appimage) MUST be present and valid.
- **REQ-INSTALL-003**: A dedicated documentation file (`install_step.md`) MUST be provided with step-by-step instructions.

### Dependencies
- **REQ-DEP-001**: The project MUST document all system dependencies required for building on Linux (e.g., `libwebkit2gtk-4.0-dev`).

### Hotkey Configuration
- **REQ-HOTKEY-001**: The documentation MUST include specific instructions for configuring the global hotkey (PrintScreen) on:
    - GNOME (Keyboard Settings)
    - Hyprland (`bind`)
    - Sway/i3 (`bindsym`)

### Updating
- **REQ-UPDATE-001**: The documentation MUST explain how to update the installed application when new code is pulled.

## Constraints

- The installation process relies on `cargo tauri build` and system package managers. We are not distributing binaries yet, so users build from source.

## Future Scope

- CI/CD pipelines for automated binary releases.
- Distribution via package managers (AUR, apt repo).
