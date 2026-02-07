## Why

The crop tool is listed in the initial specification and referenced in the annotation-tools spec, but has no implementation. Users need to trim screenshots to focus on specific regions. Without crop, users must re-capture or use external tools to trim images.

## What Changes

- Add crop tool to the tool palette UI
- Implement crop rectangle drawing, rendering with dark overlay, and resize handles
- Add "Apply Crop" workflow that trims the base image and adjusts all annotations
- Add cancel workflow to discard a crop without applying
- Support undo of crop operations via existing history system

## Capabilities

### New Capabilities
- `crop-tool`: Draw a crop rectangle to define a region, then apply to trim the screenshot and clip/remove annotations outside the crop area

## Impact

- `src/lib/components/ToolPalette.svelte` — new tool button
- `src/lib/components/Canvas.svelte` — drawing, rendering, apply/cancel logic, keyboard shortcuts
- `src/lib/utils/hitTest.ts` — crop hit testing for selection
