# Capability Spec: System Tray

## High Level Requirement
The application must be accessible via a system tray icon, allowing background operation and quick access to common actions.

## Detailed Requirements

### 1. Tray Icon Visibility
- **Requirement**: The application must display an icon in the OS system tray.
- **Requirement**: The icon should be visually distinct (target/crosshair style) and "modern".

### 2. Context Menu
- **Requirement**: Right-clicking the tray icon must open a context menu.
- **Requirement**: The menu must contain:
    -   `Show/Hide`: Toggles the main window visibility.
    -   `Quit`: Terminates the application completely.

### 3. Window Behavior
- **Requirement**: Clicking the tray icon (left click) should toggle the window visibility (Show if hidden, Hide if visible/focused).
- **Requirement**: When the window is minimized or closed by the user, it should (optionally) minimize to the tray instead of exiting, effectively running in the background. *Refinement: For this iteration, standard Minimize behavior remains OS default, but "Close" could hide to tray. Let's stick to "Tray Icon adds control" first.*
- **Requirement**: Implementing "Close minimizes to tray" is a common pattern for such apps.
    -   Interceptor: The window close event should be intercepted to hide the window instead of exiting, unless "Quit" is triggered from the tray or app menu.

### 4. Platform Support
- **Requirement**: Must work on Linux (Type: System Tray).
- **Requirement**: Should be compatible with Windows/macOS implementations provided by Tauri.
