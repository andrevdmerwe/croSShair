## Why

The canvas currently renders the captured image at a single auto-fit scale (computed in `Canvas.svelte`'s `resizeCanvas()`, capped at 1x) with no way for the user to see more detail or fit more of a large image on screen. Annotating small details (e.g. placing a step marker precisely, or drawing on a high-resolution capture) is imprecise because the user cannot magnify the working view. Zoom is a baseline expectation for any screenshot annotation tool.

## What Changes

- Add user-controlled zoom in/out on the captured image within the editor view.
- Add zoom controls (zoom in, zoom out, reset-to-fit, current zoom percentage display) to the toolbar.
- Support `Ctrl +` / `Ctrl -` / `Ctrl 0` keyboard shortcuts for zoom in, zoom out, and reset-to-fit.
- Support `Ctrl + mouse wheel` over the canvas to zoom in/out centered on the cursor position.
- Allow zooming beyond 100% (upscaling), unlike the current auto-fit scale which caps at 1x.
- Preserve existing pan-via-scroll behavior (the canvas container already scrolls when content overflows); zooming in increases scrollable area.
- Annotation hit-testing, rendering, and the in-progress text-edit overlay continue to work correctly at any zoom level (they already derive from the shared `scale` value).

## Capabilities

### New Capabilities
- `image-zoom`: Zoom in/out on the captured image via toolbar controls, keyboard shortcuts, and Ctrl+wheel, with a visible zoom percentage and a reset-to-fit action.

### Modified Capabilities
(none — existing tool-palette/canvas rendering capabilities are not formally tracked as delta specs, and no existing documented requirement changes; this change only adds new behavior)

## Impact

- `src/lib/components/Canvas.svelte`: generalize the auto-fit `scale` computation into a zoom system (min/max zoom bounds, zoom-in/out/reset functions, Ctrl+wheel handler, exposed zoom level for the toolbar).
- `src/lib/components/ToolPalette.svelte` (or main toolbar layout in `src/routes/+page.svelte`): add zoom control UI (zoom out button, percentage readout, zoom in button, reset button).
- Possibly a new small store or exported bindable state to share zoom level between `Canvas.svelte` and the toolbar, since `Canvas` is re-instantiated per-tab via `{#key tabStore.activeTabId}`.
- No backend (Rust/Tauri) changes required — this is purely a frontend rendering/interaction change.
