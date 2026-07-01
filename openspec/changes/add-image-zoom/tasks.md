## 1. Core Zoom State & Fit Logic

- [x] 1.1 In `Canvas.svelte`, extract the existing fit-scale calculation out of `resizeCanvas()` into a `computeFitScale()` helper (containerWidth/Height − padding, image aspect ratio, capped at 1x) without changing its current behavior.
- [x] 1.2 Add `isFitToWindow` (`$state`, default `true`) and zoom bound constants `MIN_ZOOM = 0.1`, `MAX_ZOOM = 5.0`.
- [x] 1.3 Update `resizeCanvas()` so it: recomputes `scale = computeFitScale()` only when `isFitToWindow` is `true`; otherwise keeps the current `scale` and only reapplies canvas `style.width/height` for the current container/image dimensions.
- [x] 1.4 Add a `setZoom(newScale: number)` helper that clamps to `[MIN_ZOOM, MAX_ZOOM]`, sets `isFitToWindow = false`, updates canvas style dimensions, and re-renders base/overlay.
- [x] 1.5 Add `zoomIn()` / `zoomOut()` (multiply/divide `scale` by `1.25`, then `setZoom`) and `resetZoomToFit()` (`scale = computeFitScale()`, `isFitToWindow = true`, reapply dimensions).

## 2. Cursor-Centered Wheel Zoom

- [x] 2.1 Add an `onwheel` handler on `.canvas-container` (or overlay canvas) that only acts when `e.ctrlKey` is true; call `e.preventDefault()` in that case.
- [x] 2.2 Compute the cursor's image-space coordinate from the current `scrollLeft/scrollTop` and `scale` before changing zoom.
- [x] 2.3 Compute the new scale via `factor = Math.pow(1.0015, -e.deltaY)`, apply through `setZoom`, clamped to bounds.
- [x] 2.4 After the scale updates, set `scrollLeft`/`scrollTop` on the container so the same image-space point stays under the cursor.
- [x] 2.5 Verify plain (non-Ctrl) wheel scroll is left untouched and still pans via native scrolling.

## 3. Keyboard Shortcuts

- [x] 3.1 In the existing `handleKeyDown` in `Canvas.svelte`, add handling for `Ctrl`/`Cmd` + (`=` or `+`) → `zoomIn()`, with `preventDefault()`.
- [x] 3.2 Add `Ctrl`/`Cmd` + `-` → `zoomOut()`, with `preventDefault()`.
- [x] 3.3 Add `Ctrl`/`Cmd` + `0` → `resetZoomToFit()`, with `preventDefault()`.
- [x] 3.4 Confirm these new shortcuts are ignored while editing text or when an input/textarea is focused (reuse the existing early-return checks).

## 4. Zoom Toolbar UI

- [x] 4.1 Add a floating zoom control bar to `Canvas.svelte`'s template (positioned similarly to the existing `.crop-controls` overlay, e.g. bottom-right of `.canvas-container`), shown whenever `imageLoaded` is true.
- [x] 4.2 Add zoom-out button, a `{Math.round(scale * 100)}%` readout, zoom-in button, and a reset-to-fit button, wired to `zoomOut()`, `zoomIn()`, `resetZoomToFit()`.
- [x] 4.3 Disable/style the zoom-out button at `MIN_ZOOM` and the zoom-in button at `MAX_ZOOM` (or clamp silently — match whichever pattern the crop controls use for disabled states, if any).
- [x] 4.4 Style the control bar to match the current theme (`var(--bg-glass)`/`var(--accent)` etc., consistent with existing overlay styling).

## 5. Verification

- [ ] 5.1 Manually test zoom in/out via buttons, keyboard shortcuts, and Ctrl+wheel at various image sizes (small screenshot, large multi-monitor capture).
- [ ] 5.2 Manually test that annotation drawing, selection, resize handles, and hit-testing remain accurate at zoom levels below 100%, at 100%, and above 100%.
- [ ] 5.3 Manually test that resizing the app window re-fits the image when in fit-to-window mode, and does NOT change zoom after a manual zoom action.
- [ ] 5.4 Manually test the text annotation editing overlay positions/sizes correctly at a non-100% zoom level.
- [x] 5.5 Run `npm run check` (or existing type-check/lint script) to confirm no TypeScript/Svelte errors were introduced.
