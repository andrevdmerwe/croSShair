# Clipboard Integration

## Summary
The system must allow users to copy the current visual state of the canvas (composited) directly to the system clipboard for immediate pasting into other applications.

## Requirements

### Functional Requirements
- **Copy to Clipboard**: A "Copy" action MUST place the flattened image (PNG blob) onto the clipboard.
- **Format**: The clipboard content MUST be `image/png`.

### System Constraints
- **Linux/Wayland**: The implementation MUST work on common Linux compositors (GNOME, KDE) using standard tools like `wl-copy`.
- **Dependencies**: The system relies on external clipboard utilities (`wl-copy`) which should be verified at runtime.

### Non-Functional Requirements
- **Feedback**: Is critical to show a transient success message ("Copied to clipboard") or error if it fails.

## Edge Cases
- **Tool Missing**: If `wl-copy` is missing, the action should fail gracefully with a user notification.
