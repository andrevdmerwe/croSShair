# Capability Spec: Focus Handling

## Requirements

### REQ-FOCUS-001: No Focus Steal on Capture
- **WHEN** the application is launched with `capture-region` argument
- **THEN** the application window MUST NOT steal focus from the currently active window

### REQ-FOCUS-002: Normal Focus on Direct Launch
- **WHEN** the application is launched without arguments (e.g., from app launcher)
- **THEN** the application window SHOULD receive focus as normal

## Constraints
- Must work on Wayland compositors (Niri, Hyprland, GNOME).
- Tauri's `WindowBuilder` has a `focused(bool)` option.
