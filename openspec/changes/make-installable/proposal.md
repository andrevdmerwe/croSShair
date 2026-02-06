# Make croSShair Installable and Document Installation

## Problem Description

The current codebase lacks explicit documentation and tooling for end-users to install the application locally. While `cargo tauri build` works, users need guidance on prerequisites, build steps, and post-installation configuration (specifically global hotkeys). Additionally, there is no standardized process for updating the installed application when code changes.

## Proposed Solution

We will enhance the project to support local installation and provide comprehensive documentation.

### 1. Installation Documentation (`install_step.md`)
Create a dedicated guide covering:
- **Prerequisites**: Node.js, Rust/Cargo, and system dependencies (webkit2gtk, etc.).
- **Installation**: Steps to build the release binary and install it (e.g., using `dpkg` or copying the binary).
- **Global Hotkey**: Instructions for configuring the desktop environment (GNOME/Wayland) to bind a key (e.g., PrintScreen) to the app.
- **Updates**: A workflow for pulling changes, rebuilding, and reinstalling.

### 2. Bundle Configuration
Update `src-tauri/tauri.conf.json` to ensure proper bundling:
- Verify `bundle.identifier` and `bundle.targets` (deb, appimage).
- Ensure icons are correctly configured.

### 3. Installation Script (Optional but recommended)
Create a helper script `scripts/install.sh` to automate the build-and-install process for Linux users, simplifying the "update" workflow.

## Acceptance Criteria

- [ ] `install_step.md` exists and accurately documents the installation process.
- [ ] Users can successfully build and install the app using the provided instructions.
- [ ] Documentation includes clear steps for configuring global hotkeys on common Wayland compositors (GNOME, Hyprland, Sway).
- [ ] `src-tauri/tauri.conf.json` is configured to produce valid Linux bundles (deb).
