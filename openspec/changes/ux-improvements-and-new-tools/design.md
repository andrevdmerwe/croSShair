# Design: UX Improvements and New Tools

## Architecture

Changes span frontend (Svelte), backend (Rust/Tauri), and state management (stores).

### 1. Focus Handling (Rust/Tauri)
- Modify window creation in `lib.rs` to set `focused: false` when `capture-region` intent is detected.
- This prevents the app from stealing focus when launched via hotkey.

### 2. Tool State Reset (Frontend)
- Add `resetToDefaults()` function in `tools.svelte.ts`.
- In `tabs.svelte.ts`, detect when `tabs` array becomes empty and call `toolStore.resetToDefaults()`.

### 3. Highlight Tool (Frontend)
- New annotation type: `highlight`
- Renders as a semi-transparent filled rectangle (default: yellow, 30% opacity).
- Properties: `x`, `y`, `width`, `height`, `color`, `opacity`.
- Add to `ToolPalette.svelte` and `renderAnnotationOnContext`.

### 4. Speech Bubble Tool (Frontend)
- New annotation type: `speechBubble`
- Renders as a rounded rectangle with a triangular pointer and text.
- Properties: `x`, `y`, `width`, `height`, `text`, `pointerX`, `pointerY`, `color`, `textColor`.
- Requires text input similar to existing Text tool.
- Add to `ToolPalette.svelte` and `renderAnnotationOnContext`.

## Data Model

### New Annotation Types
```typescript
// Highlight
{ type: 'highlight', x, y, width, height, color, data: { opacity: 0.3 } }

// Speech Bubble
{ type: 'speechBubble', x, y, width, height, color, data: { text, pointerX, pointerY, textColor } }
```

## Migration Strategy
- No database changes required.
- Annotations are stored in-memory per tab.
