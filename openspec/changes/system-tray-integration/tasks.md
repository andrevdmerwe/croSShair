# Tasks: System Tray Integration

## Build & Dependencies
- [x] Add `tauri-plugin-tray` to `src-tauri/Cargo.toml` <!-- id: 1 -->
- [x] Add `tauri-plugin-tray = "2"` to `[dependencies]` <!-- id: 2 -->

## Backend Implementation
- [x] Initialize Tray Plugin in `src-tauri/src/lib.rs` <!-- id: 3 -->
    - [x] Import `tauri::menu::{Menu, MenuItem}` and `tauri::tray::TrayIconBuilder`.
    - [x] Create a `Menu` with "Show/Hide" and "Quit".
    - [x] Add `.plugin(tauri_plugin_tray::init())` or manual builder configuration if custom logic is needed (Builder is preferred for menu handling).
    - [x] Removed `tauri-plugin-tray` dependency and init in favor of `tauri` crate `tray-icon` feature for Rust-native handling.
- [x] Implement Menu Event Handling <!-- id: 4 -->
    - [x] `on_menu_event`: Handle "quit" -> `app.exit()`.
    - [x] `on_menu_event`: Handle "toggle" -> check window visibility and toggle.
- [x] Implement Tray Icon Click Handlers <!-- id: 5 -->
    - [x] `on_tray_icon_event`: Handle `LeftClick` -> toggle window visibility.
- [x] Implement Window Close Interception <!-- id: 6 -->
    - [x] In `run()` builder, listen for `WindowEvent::CloseRequested`.
    - [x] Prevent default close.
    - [x] Call `window.hide()`.

## Assets
- [x] Verify `icons/icon.png` is compatible or specific tray icon usage. <!-- id: 7 -->

## Verification
- [ ] Verify tray icon appears on launch. <!-- id: 8 -->
- [ ] Verify Left Click toggles window. <!-- id: 9 -->
- [ ] Verify Right Click menu shows "Show/Hide" and "Quit". <!-- id: 10 -->
- [ ] Verify "Quit" actually closes the application. <!-- id: 11 -->
- [ ] Verify closing the window (X button) hides it to tray instead of exiting. <!-- id: 12 -->
