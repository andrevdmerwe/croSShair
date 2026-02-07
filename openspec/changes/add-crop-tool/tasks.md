## 1. Tool Palette

- [ ] 1.1 Add crop tool button with icon to ToolPalette.svelte tools array

## 2. Hit Testing

- [ ] 2.1 Add crop case to hitTestAnnotation switch in hitTest.ts (reuse hitTestRectangle)

## 3. Drawing Logic

- [ ] 3.1 Add single-crop enforcement: remove existing crop before creating new one
- [ ] 3.2 Add crop to mouse move drawing condition (rectangle-style width/height update)
- [ ] 3.3 Auto-select crop annotation after drawing completes

## 4. Rendering

- [ ] 4.1 Add crop case to renderAnnotationOnContext with dark overlay and dashed border
- [ ] 4.2 Ensure crop renders last in overlay (defer rendering in renderOverlay)

## 5. Apply/Cancel UI

- [ ] 5.1 Add cropAnnotation derived state
- [ ] 5.2 Add Apply Crop and Cancel buttons in canvas template
- [ ] 5.3 Style crop control buttons with theme variables

## 6. Crop Application Logic

- [ ] 6.1 Implement applyCrop function: crop base image via offscreen canvas
- [ ] 6.2 Implement annotation transformation: offset coordinates, clip/remove out-of-bounds
- [ ] 6.3 Implement cancelCrop function

## 7. Keyboard Shortcuts

- [ ] 7.1 Add Enter to apply crop and Escape to cancel crop in handleKeyDown
