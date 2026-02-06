# design.md

## Architecture

### CSS Architecture
We will use a **Global CSS Variables** approach.
-   `src/app.css` (or `src/theme.css`): Defines `:root` with default (Dark) variables.
-   Defines `[data-theme="light"]` selector to override variables for Light mode.

**Core Tokens:**
```css
:root {
  --bg-primary: #1a1a1a;
  --bg-secondary: #252525;
  --bg-glass: rgba(30, 30, 30, 0.8);
  --text-main: #ffffff;
  --text-muted: #a0a0a0;
  --accent: #ff4757; /* Red crosshair brand color? */
  --border: #333333;
  --radius-md: 8px;
  --shadow-lg: 0 10px 15px -3px rgba(0, 0, 0, 0.5);
}
```

### State Management
-   **`theme.svelte.ts`**: A robust store using Svelte 5 runes.
    -   `currentTheme`: string ('dark' | 'light').
    -   `toggleTheme()`: switches and saves to localStorage.
    -   `init()`: Reads localStorage or system preference on load.
    -   `customOverrides`: Object storing user-defined color overrides.

### Component Updates

#### `src-tauri/tauri.conf.json`
-   Set `"decorations": false` to remove OS title bar and borders.
-   Ensure `"transparent": true` (optional, but good for rounded corners if needed later).

#### `ToolPalette.svelte` / `Toolbar.svelte`
-   Wrap contents in a container with:
    -   `position: fixed; bottom: 20px; left: 50%; transform: translateX(-50%);`
    -   `background: var(--bg-glass); backdrop-filter: blur(10px);`
    -   `border-radius: var(--radius-md);`
    -   `box-shadow: var(--shadow-lg);`

#### `SettingsModal.svelte`
-   Update structure to use a clean grid layout.
-   Add "Theme" dropdown populated from a constant list of themes.
-   Add Color Pickers for `accent`, `bg-primary`, `text-main` that update `themeStore.customOverrides`.

#### `Canvas.svelte`
-   Update `renderOverlay` and cursor logic to reference distinct colors (maybe hardcoded high-contrast is safer for canvas, but UI elements like the textarea MUST match theme).

## Migration Strategy
1.  **Foundation**: Create `theme.css` and `theme.svelte.ts`.
2.  **Replacement**: Grep for hex codes in components and replace with `var(...)` one by one.
3.  **Restyle**: Apply new layout styles to Toolbar/Settings.
4.  **Cleanup**: Delete unused CSS.
