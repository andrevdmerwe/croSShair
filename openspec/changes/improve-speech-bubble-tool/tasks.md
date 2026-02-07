## 1. Unified Shape Rendering

- [x] 1.1 Replace 3-step speechBubble rendering in `renderAnnotationOnContext` with single composite path (rounded rect body + integrated pointer) in `Canvas.svelte`
- [x] 1.2 Implement corner radius calculation (`Math.min(12, nw * 0.15, nh * 0.15)`)
- [x] 1.3 Implement pointer base clamping within bottom edge between corner arcs

## 2. Draw Interaction

- [x] 2.1 Change mouseDown to start with `width=0, height=0` and no pointer data
- [x] 2.2 Add pointer auto-positioning in mouseMove during creation (center below bubble, offset proportional to height)
- [x] 2.3 Add minimum size enforcement (60x30) on mouseUp for click-without-drag

## 3. Hit Testing & Selection

- [x] 3.1 Add `hitTestSpeechBubble` function with body rectangle + pointer triangle check in `hitTest.ts`
- [x] 3.2 Add `pointInTriangle` and `triangleSign` helper functions
- [x] 3.3 Add `speechBubble` case to `getAnnotationBounds` that includes pointer in bounding box

## 4. Verification

- [ ] 4.1 Test click-and-drag creation expands from click point
- [ ] 4.2 Test click-without-drag creates minimum-sized bubble with text editing
- [ ] 4.3 Test pointer is visually connected with no border gap
- [ ] 4.4 Test clicking pointer triangle area selects annotation
- [ ] 4.5 Test selection outline encompasses both body and pointer
- [ ] 4.6 Test pointer handle drag and body drag behaviors preserved
- [ ] 4.7 Test with multiple themes
