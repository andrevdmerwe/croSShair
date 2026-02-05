# design.md

## Architecture
We will use `tauri-plugin-tray` to handle system tray interactions. The logic will be implemented primarily in the Rust backend (`src-tauri/src/lib.rs` or `main.rs`) to ensure robust OS integration.

## Dependencies
-   `tauri-plugin-tray` (add to `Cargo.toml`).
-   Icon assets (png/ico) for the tray.

## Components

### 1. `src-tauri/Cargo.toml`
-   Add `tauri-plugin-tray`.

### 2. `src-tauri/src/lib.rs`
-   Initialize the tray plugin in the `Builder`.
-   Define menu items: `Show`, `Hide`, `Quit`.
-   Handle `TrayIconEvent` (Left Click, Right Click) and `MenuEvent`.

### 3. Window Event Handling (`lib.rs`)
-   Intercept `WindowEvent::CloseRequested`.
-   Instead of closing, call `window.hide()`.
-   Ensure `Quit` menu item calls `app.exit()` to purely close the app.

### 4. Icon Asset
-   Use `icons/icon.png` (or a specific tray variant if needed, e.g., monochrome for macOS, but color for Linux is fine).
-   User requested "modern stylish tray icon". We will use the existing app icon for now but ensure it's loaded correctly. *Self-correction: User specifically asked for a NEW icon. I should try to generate one or assume existing one is insufficient. I will stick to code enablement first, but I'll check if I can update the icon resource.*

## Data Flow
User Action (Click Tray) -> Rust Event Loop -> Toggle Window Visibility.
User Action (Click Close Window) -> Rust Event Loop -> Window Hide.
User Action (Click Quit Tray) -> Rust Event Loop -> App Exit.

## Testing Strategy
-   Run `npm run tauri dev`.
-   Verify Icon appears.
-   Verify Left Click toggles window.
-   Verify "Quit" closes app.
-   Verify "X" button hides window (if implemented) or minimizes.
