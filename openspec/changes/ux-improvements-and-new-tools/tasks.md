# Tasks: UX Improvements and New Tools

## 1. Focus Handling
- [ ] Modify `lib.rs` to set `focused: false` on window creation when `capture-region` intent is detected.
- [ ] Verify capture via hotkey does not steal focus.

## 2. Tool State Reset
- [ ] Add `resetToDefaults()` function to `tools.svelte.ts`.
- [ ] In `tabs.svelte.ts`, call `toolStore.resetToDefaults()` when tabs array becomes empty.
- [ ] Verify closing last tab resets tool, color, and line width.

## 3. Highlight Tool
- [ ] Add `highlight` to `AnnotationType` in types.
- [ ] Add Highlight button to `ToolPalette.svelte`.
- [ ] Implement drawing logic in `Canvas.svelte` for highlight tool.
- [ ] Implement `highlight` rendering in `renderAnnotationOnContext`.
- [ ] Add `highlight` case to `hitTestAnnotation` in `hitTest.ts`.
- [ ] Verify highlight tool is selectable, movable, resizable, and deletable.

## 4. Speech Bubble Tool
- [ ] Add `speechBubble` to `AnnotationType` in types.
- [ ] Add Speech Bubble button to `ToolPalette.svelte`.
- [ ] Implement drawing logic in `Canvas.svelte` for speech bubble (initial position + pointer).
- [ ] Implement text input for speech bubble (similar to Text tool).
- [ ] Implement `speechBubble` rendering in `renderAnnotationOnContext` (rounded rect + triangle + text).
- [ ] Add `speechBubble` case to `hitTestAnnotation` in `hitTest.ts`.
- [ ] Implement pointer handle for independent pointer movement.
- [ ] Verify speech bubble tool is selectable, movable, resizable, and deletable.

## 5. Verification
- [ ] Test all existing tools still work correctly.
- [ ] Test capture via hotkey does not steal focus.
- [ ] Test closing last tab resets tool state.
- [ ] Test highlight tool end-to-end.
- [ ] Test speech bubble tool end-to-end.
