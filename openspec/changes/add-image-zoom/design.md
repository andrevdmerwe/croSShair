## Context

`Canvas.svelte` already renders the captured image at a variable `scale` (see `resizeCanvas()` around line 784): the canvas element's internal resolution always matches the image's natural pixel dimensions, while `canvas.style.width/height` apply `scale` for on-screen display. Every downstream consumer of coordinates — mouse-to-image conversion (`(e.clientX - rect.left) / scale`), resize-handle sizing (`8 / scale`), hit-test thresholds (`6 / scale`), and the floating text-edit `<textarea>` overlay position/size — already derives from this single `scale` value. The `.canvas-container` wrapper already has `overflow: auto`, so panning a zoomed-in image via native scrollbars/trackpad already works with no changes.

Today, `scale` is fully recomputed by `resizeCanvas()` on every image load *and* on every `ResizeObserver` firing (e.g. window resize, sidebar toggle), always as "fit to container, capped at 1x". There is no user control over it, and no upscaling beyond natural size.

`Canvas.svelte` is destroyed and recreated per tab switch (`{#key tabStore.activeTabId}` in `+page.svelte`), and it is a sibling of `ToolPalette.svelte`, not a parent/child. The only existing cross-component bridge is `bind:this={currentCanvas}` exposing instance methods like `exportImage()`.

## Goals / Non-Goals

**Goals:**
- Let the user zoom in/out on the canvas beyond the current fixed auto-fit scale, including upscaling past 100%.
- Provide toolbar buttons, keyboard shortcuts (`Ctrl +`/`Ctrl -`/`Ctrl 0`), and `Ctrl+wheel` (cursor-centered) as three equivalent ways to change zoom.
- Keep all existing scale-dependent logic (hit-testing, handle sizes, text overlay) working unchanged, since they already key off `scale`.
- Preserve current "fit to window" behavior as the default/reset state.

**Non-Goals:**
- Persisting zoom level across tab switches or app restarts (each tab still opens at fit-to-window; this is a possible future enhancement, not required here).
- Touch/pinch-to-zoom gestures (out of scope; Wayland desktop app, mouse/trackpad + keyboard only).
- Changing the crop tool's own zoom/preview behavior beyond continuing to work at any canvas zoom level.

## Decisions

**1. Zoom controls live inside `Canvas.svelte` as a self-contained overlay, not in `ToolPalette.svelte`.**
`Canvas` already renders a similar floating overlay for crop (`.crop-controls`, positioned absolutely within `.canvas-stack`/`.canvas-container`). Zoom controls (zoom out / percentage / zoom in / reset-to-fit) follow the same pattern: a small floating control bar (e.g. bottom-right of `.canvas-container`). This avoids introducing a new store or bindable prop just to sync a value that resets every tab switch anyway, and keeps the change localized to one file.
- *Alternative considered*: expose zoom level as a bindable/exported value from `Canvas` and render controls in `ToolPalette`. Rejected — `ToolPalette` is a sibling of `Canvas`, not a parent, so sharing live reactive state would require a new store (e.g. a per-tab zoom map) purely for UI placement, adding complexity disproportionate to the benefit. Can revisit if a future requirement needs zoom controls outside the canvas area.

**2. Split `scale` into an explicit fit-scale calculation plus a live, user-adjustable value; track whether the user has manually zoomed.**
Add a `isFitToWindow` boolean (`$state`, default `true`). `resizeCanvas()` is split so that:
- The fit calculation (existing `containerWidth/Height` + `Math.min(scale, 1)` logic) becomes a helper `computeFitScale()`.
- On image load: `scale = computeFitScale()`, `isFitToWindow = true`.
- On `ResizeObserver` firing: if `isFitToWindow` is true, recompute `scale = computeFitScale()` (preserves today's behavior of tracking window size); if false (user has manually zoomed), leave `scale` untouched and only reapply it to the canvas style dimensions.
- Any manual zoom action (buttons, keyboard, wheel) sets `isFitToWindow = false`. The reset-to-fit action sets `scale = computeFitScale()` and `isFitToWindow = true`.
- *Alternative considered*: always re-fit on container resize regardless of manual zoom. Rejected — this would silently discard the user's zoom whenever the window resizes or the sidebar is toggled, which is surprising and defeats the point of manual zoom.

**3. Zoom bounds and step size.**
- Min zoom: `0.1` (10%), max zoom: `5.0` (500%). Clamp on every zoom operation. Rationale: keeps canvas style dimensions and hit-test math within sane bounds and matches common image-editor ranges; large enough for pixel-level annotation, small enough to always keep a huge screenshot visible.
- Discrete step (buttons + keyboard shortcuts): multiply/divide `scale` by `1.25` per action, then clamp.
- Wheel zoom: continuous factor based on `e.deltaY`, e.g. `factor = Math.pow(1.0015, -e.deltaY)`, then clamp — gives proportionally smaller steps for typical mouse wheel ticks and smoother response for trackpad scroll deltas.

**4. Cursor-centered wheel zoom via scroll-offset correction.**
Only intercept wheel events when `e.ctrlKey` is true (the standard convention browsers/trackpads use to signal "zoom", already distinct from normal two-finger scroll/pan). On such an event: read the cursor position relative to `.canvas-container`, convert to image-space using the *current* scale and current `scrollLeft/scrollTop`, compute the new scale, then set `scrollLeft/scrollTop` so that the same image-space point remains under the cursor. `preventDefault()` is required to stop the WebView's native page-zoom.

**5. Keyboard shortcuts added to the existing global `handleKeyDown` in `Canvas.svelte`.**
Reuse the existing `(e.ctrlKey || e.metaKey)` pattern already used for undo/redo. Handle `=`/`+` (zoom in — both keys because `+` requires Shift on most layouts), `-` (zoom out), and `0` (reset to fit), with `preventDefault()` to block the WebView's native zoom shortcuts.

## Risks / Trade-offs

- **[Risk]** Very large source images at high zoom (e.g. 500% on a 4K capture) could produce a very large `canvas.style.width/height`, impacting layout/scroll performance. → **Mitigation**: canvas *internal* resolution never changes (still native image size); only CSS display size grows, which browsers handle cheaply via compositing. Max zoom is capped at 5x.
- **[Risk]** `Ctrl+wheel`/`Ctrl+=`/`Ctrl+-` can collide with the WebView's own zoom-page shortcuts. → **Mitigation**: `preventDefault()` on every handled case, consistent with how undo/redo already intercept `Ctrl+Z`.
- **[Trade-off]** Zoom resets to fit-to-window on every tab switch (since `Canvas` remounts per tab). Accepted as a non-goal for this change; documented above.

## Open Questions

- None blocking implementation. Per-tab zoom persistence (listed as a non-goal) can be revisited later if requested.
