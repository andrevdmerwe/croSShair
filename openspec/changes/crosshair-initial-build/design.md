## Context

Building a new screenshot and annotation tool from scratch for CachyOS Linux with Niri Wayland compositor. The Wayland ecosystem provides mature CLI tools (`grim`, `slurp`, `wl-copy`) that handle the low-level display capture, so the application can focus on the editing experience.

No existing codebase—this is a greenfield project using Tauri for the desktop shell.

## Goals / Non-Goals

**Goals:**
- Create a lightweight, fast-launching screenshot tool optimized for Wayland
- Provide comprehensive annotation capabilities in a tabbed editor interface
- Integrate seamlessly with Niri keybindings via CLI interface
- Support XDG-compliant configuration storage

**Non-Goals:**
- Full-screen or active-window capture modes (only region capture)
- Custom theming or visual polish (functionality first)
- Support for X11 or non-Wayland environments
- Cloud sync or remote storage features

## Decisions

### Frontend Framework: Svelte
**Rationale**: Svelte compiles to vanilla JavaScript with minimal runtime overhead, resulting in faster startup times and lower memory usage compared to React or Vue. For a screenshot tool that may be invoked frequently via keybindings, startup performance is critical.
**Alternatives**: React (larger bundle, virtual DOM overhead), Vue (similar trade-offs to React).

### Canvas Rendering: HTML5 Canvas API
**Rationale**: Native browser Canvas API provides hardware-accelerated 2D rendering without external dependencies. For the annotation tools (shapes, text, freehand), Canvas offers the necessary primitives with predictable performance.
**Alternatives**: SVG (better for resolution-independent graphics but more complex hit-testing), WebGL (overkill for 2D annotation).

### Single-Instance with IPC
**Rationale**: When user triggers capture via CLI (`croSShair capture-region`), if app is already running, send the new screenshot to existing instance rather than launching a new window. Tauri's single-instance plugin handles this.
**Alternatives**: Always launch new instance (wastes resources), D-Bus integration (more complex, Tauri plugin is simpler).

### Layer-Based Canvas Architecture
**Rationale**: Separate base screenshot layer from annotation overlays allows non-destructive editing and efficient undo/redo. Only pixelation and crop operations modify the base layer.
**Alternatives**: Flatten all operations immediately (loses undo capability), full object model (more complex than needed).

## Risks / Trade-offs

**[WebView performance on Wayland]** → Tauri uses WebKitGTK which has good Wayland support. Enable GPU acceleration via Tauri config. Monitor startup time during development.

**[CLI tool availability]** → Users may not have grim/slurp/wl-clipboard installed. Check on startup and show clear installation guidance. Allow viewing/editing existing images even without capture tools.

**[Large image handling]** → High-resolution screenshots may strain Canvas performance. Consider downscaling for display while maintaining full resolution for export. Defer optimization until proven necessary.

**[Single-instance IPC failure]** → If IPC fails, fall back to opening new instance rather than failing silently. Log the error for debugging.
