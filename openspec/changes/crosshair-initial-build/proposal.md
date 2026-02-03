## Why

Building **croSShair**, a lightweight screenshot and annotation tool for CachyOS Linux with Niri Wayland compositor. The current Linux screenshot tool landscape lacks a cohesive solution that combines region capture, tabbed editing, and comprehensive annotation tools optimized for Wayland—specifically Niri's wl-roots-style environment.

## What Changes

- Create a new Tauri-based desktop application with web frontend (React/Svelte/Vue)
- Integrate `grim`, `slurp`, and `wl-copy` CLI tools for Wayland screenshot capture
- Implement a tabbed interface for managing multiple screenshot editing sessions
- Build a complete annotation toolkit: freehand drawing, highlighter, shapes, arrows, text, step markers, pixelation, and cropping
- Implement auto-save on capture and manual save with dirty state tracking
- Add clipboard integration with optional auto-copy after capture
- Provide CLI interface (`croSShair capture-region`) for Niri keybinding integration
- Create settings subsystem with XDG-compliant configuration storage

## Capabilities

### New Capabilities
- `screenshot-capture`: Region capture pipeline using grim/slurp with auto-save and clipboard support
- `tabbed-editor`: Browser-style tabbed interface for managing multiple screenshot sessions
- `annotation-tools`: Canvas-based annotation system with shapes, text, markers, and pixelation
- `settings-config`: Configuration management with XDG-compliant storage and settings UI
- `cli-integration`: Command-line interface for compositor keybinding integration

### Modified Capabilities
<!-- No existing capabilities to modify - this is a new project -->

## Impact

- **New codebase**: Full Tauri application with Rust backend and web frontend
- **External dependencies**: Requires `grim`, `slurp`, `wl-clipboard` packages on the system
- **Configuration**: Creates `~/.config/croSShair/config.json` for persistent settings
- **File system**: Writes screenshots to configurable directory (default: `~/Pictures/Screenshots`)
