# Tasks: UI Modernization and Theming

## Foundation
- [ ] Create `src/theme.css` with `:root` variables for colors, spacing, and typography (Light & Dark modes) <!-- id: 1 -->
- [ ] Create `src/lib/stores/theme.svelte.ts` with `themeStore` (runes) for state management and persistence <!-- id: 2 -->
    - [ ] Implement `currentTheme` and `customOverrides`.
    - [ ] Define Theme Definitions (Macchiato, Nord, Osaka Jade, Kanagawa, Tokyo Night).
- [ ] Import `theme.css` in `src/routes/+layout.svelte` (or root component) <!-- id: 3 -->
- [x] Configure `src-tauri/tauri.conf.json` <!-- id: 13 -->
    - [x] Set `"decorations": false`.
    - [x] Set `"transparent": true` (if supported/desired for rounded aesthetics).

## Component Modernization
- [x] Update `Toolbar.svelte` & `ToolPalette.svelte` <!-- id: 4 -->
    - [x] Apply glassmorphism background (`backdrop-filter`).
    - [x] Use theme variables for colors.
    - [x] Improve spacing and border-radius.
- [x] Rebuild `SettingsModal.svelte` <!-- id: 5 -->
    - [x] Implement new grid layout.
    - [x] Add **Theme Selector** (Dropdown).
    - [x] Add **Color Customization** (Pickers for overrides).
    - [x] Apply theme tokens to inputs/buttons.

## Canvas Integration
- [ ] Refactor `Canvas.svelte` overlays <!-- id: 6 -->
    - [ ] Replace hardcoded hex values (selection box, text tool) with CSS variables or `themeStore` derived values.

## Cleanup
- [ ] Audit and remove unused CSS from `src/app.css` and component `<style>` blocks <!-- id: 7 -->

## Verification
- [ ] Verify Light/Dark theme toggle works and persists <!-- id: 8 -->
    - [ ] Verify custom color overrides work <!-- id: 11 -->
    - [ ] Verify Frameless window behavior <!-- id: 12 -->
- [ ] Verify Toolbar looks correct in all themes <!-- id: 9 -->
- [ ] Verify Canvas overlays are visible in all themes <!-- id: 10 -->
