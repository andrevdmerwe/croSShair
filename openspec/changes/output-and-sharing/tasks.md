# Tasks

- [ ] **Implementation: Backend (Rust)** <!-- id: 1 -->
    - [x] Create `src-tauri/src/export.rs` module. <!-- id: 2 -->
    - [x] Implement `save_image` command (decode Base64 -> file write). <!-- id: 3 -->
    - [x] Implement `copy_image_to_clipboard` command (decode Base64 -> wl-copy). <!-- id: 4 -->
    - [ ] Register commands in `main.rs`. <!-- id: 5 -->

- [ ] **Implementation: Frontend (Canvas)** <!-- id: 6 -->
    - [ ] Implement `exportImage()` in `Canvas.svelte` (compositing logic). <!-- id: 7 -->
    - [ ] Verify compositing correctness (transparency, resolution). <!-- id: 8 -->

- [ ] **Implementation: Frontend (UI)** <!-- id: 9 -->
    - [ ] Update `ToolPalette.svelte` with `onSave` and `onCopy` props. <!-- id: 10 -->
    - [ ] Add Save (Disk) and Copy (Clipboard) buttons to Palette. <!-- id: 11 -->
    - [ ] Update `+page.svelte` to handle events and invoke backend. <!-- id: 12 -->

- [ ] **Verification** <!-- id: 13 -->
    - [ ] Verify Save Image creates a valid PNG file on disk. <!-- id: 14 -->
    - [ ] Verify Copy Image works with `wl-paste` or pasting into another app. <!-- id: 15 -->
