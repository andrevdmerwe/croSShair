## Context

Currently, the application has hardcoded defaults for tools (Pen/Red/2px) and a fixed palette. Screenshot storage defaults to a standard location or prompts every time (depending on OS). Users request a way to persist their preferences so that the app opens in their preferred state. This requires introducing a persistence layer for user settings.

## Goals / Non-Goals

**Goals:**
- Persist user preferences for Default Tool, Stroke Width, Color, and Pixel Density.
- Persist a customizable color palette for the toolbar.
- Persist a default directory for saving screenshots.
- Provide a UI (modal) to modify these settings.
- Apply these settings on application startup.

**Non-Goals:**
- Syncing settings across devices (local storage only).
- Profiles/multiple configurations.

## Decisions

### 1. Persistence Layer: `tauri-plugin-store` vs `localStorage`
- **Decision**: Use `localStorage` for simplicity initially.
- **Rationale**: We are building a web-based UI wrapped in Tauri. `localStorage` is sufficient for simple JSON preferences and requires no extra Rust dependencies or file system permissions logic for the MVP. If we need to share settings with the Rust backend directly later (e.g., tailored tray menus), we can migrate to `tauri-plugin-store`.
- **Alternatives**: `tauri-plugin-store` (more robust, file-based), SQLite (overkill).

### 2. Settings Management: `SettingsStore.svelte`
- **Decision**: Create a global Svelte 5 reactive store `settingsStore` that syncs with `localStorage`.
- **Rationale**: Svelte 5 runes (`$state`) provide a clean way to manage global state. An effect (`$effect`) can automatically sync changes to `localStorage`.

### 3. UI Location: Settings Modal
- **Decision**: A "gear" icon in the sidebar or bottom of the toolbar that opens a modal overlay.
- **Rationale**: Keeps the main canvas clean. Familiar pattern for desktop apps.

### 4. Palette Customization UX
- **Decision**: In the Settings modal, show the list of 8 palette colors with color pickers for each.
- **Rationale**: Simple substitution mapping.

## Risks / Trade-offs

- **[Risk]**: `localStorage` clearing.
    - **Mitigation**: Acceptable for MVP. Users rarely clear app data for installed desktop apps.
- **[Risk]**: Invalid/Stale state in `localStorage`.
    - **Mitigation**: `SettingsStore` will validate/sanitize loaded data against default fallbacks.

## Migration Plan

- N/A - New feature. Code will default to hardcoded values if no settings found (backward compatible).

## Open Questions

- None.
