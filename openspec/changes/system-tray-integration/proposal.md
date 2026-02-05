# Proposal: System Tray Integration

## Problem
The users cannot currently minimize the application to the system tray, which is a standard feature for screenshot and utility applications. Additionally, there is no way to quit the application other than closing the window.

## Solution
Implement system tray integration using `tauri-plugin-tray`.
- Add a tray icon.
- Add a context menu with "Show/Hide" and "Quit" options.
- Allow the window to be hidden to the tray instead of closing when the close button is clicked (optional or default behavior for utilities).

## User Integration
The user will see a new tray icon (a target/crosshair style). Clicking it will toggle window visibility. Right-clicking will show a menu.

## Goals
1.  Add `tauri-plugin-tray` dependency.
2.  Configure the tray icon.
3.  Implement tray menu logic (Show/Hide, Quit).
4.  Handle window close event to minimize to tray if desired.

## Non-Goals
-   Global hotkeys for show/hide (already handled by Tauri config or OS).
-   Fancy animated tray icons.
