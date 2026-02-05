# Image Export

## Summary
The system must allow users to save the current visual state of the canvas (compositing the base image and all vector annotations) to a local file.

## Requirements

### Functional Requirements
- **Flattening**: The export MUST be a single raster image (PNG) containing both the background screenshot and all visible annotations.
- **Resolution**: The export MUST maintain the original resolution of the screenshots, regardless of the current window or zoom level.
- **Save As**: Users MUST be able to choose a new destination path for the file via a system dialog.
- **Formats**: The system MUST support saving as PNG.

### Non-Functional Requirements
- **Performance**: The flattening process should take less than 500ms for a 4K image.
- **Consistency**: The exported image MUST visually match the editor view (wysiwyg).

## Edge Cases
- **No Annotations**: Saving an image with no annotations essentially effectively duplicates the original screenshot.
- **Transparency**: If the canvas has transparent areas (cropping/erasing), the output PNG should preserve transparency.
