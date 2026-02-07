## Why

The speech bubble annotation tool has three visual and interaction defects that make it look unprofessional: the border cuts through the pointer base (making body and pointer appear as separate objects), the pointer is disconnected during creation, and the drawing interaction doesn't match other shape tools (it creates a fixed-size shape on click instead of expanding from the click point).

## What Changes

- Replace the 3-step rendering (ellipse + fill triangle + stroke triangle edges) with a single unified path that draws a rounded rectangle body with an integrated pointer — one fill, one stroke, no border gap
- Change the body shape from ellipse to rounded rectangle for a more modern look
- Make the draw interaction match rectangle/ellipse: click sets origin, drag expands the shape
- Auto-position the pointer below the bubble center during creation, updating dynamically as the user drags
- Add minimum size enforcement (60x30) for click-without-drag to prevent invisible bubbles
- Add hit testing for the pointer triangle area so clicking on it selects the annotation
- Expand bounding box calculation to include the pointer position

## Capabilities

### New Capabilities

- `speech-bubble-rendering`: Unified rounded-rectangle + pointer path rendering with seamless border connection

### Modified Capabilities

- `annotation-tools`: Speech bubble draw interaction now matches rectangle/ellipse pattern (expand from click point)

## Impact

- `src/lib/components/Canvas.svelte` — Rendering, mouseDown, mouseMove, mouseUp handlers for speechBubble type
- `src/lib/utils/hitTest.ts` — Hit testing and bounding box calculation for speechBubble
- No API, dependency, or type changes required
