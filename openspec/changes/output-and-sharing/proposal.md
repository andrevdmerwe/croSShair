# Output and Sharing Features

## Goal

Enable users to export their annotated screenshots by saving them to disk ("burning" the annotations into the image) or copying the flattened result to the system clipboard.

## Problem

Currently, `croSShair` can capture an image and allow the user to draw annotations on it. However, these annotations exist only in the application's runtime memory. There is no way to output the final result. The screenshot file on disk remains the original raw capture, and the clipboard contains only the raw image (if configured). This limits the utility of the tool for sharing feedback or bug reports.

## What Changes

We will introduce output mechanisms to the `ToolPalette`:
1.  **Save Image**: A mechanism to composite the base image and the SVG/Canvas annotations into a single PNG file.
    - Options to "Save As..." (new file) or potentially overwrite the original.
2.  **Copy to Clipboard**: A mechanism to perform the same composition in memory and write the resulting image blob to the clipboard.

## Capabilities

### New Capabilities
- `image-export`: Logic for flattening the layered canvas (base image + annotations) and writing it to a file.
- `clipboard-integration`: specialized logic for writing image blobs to the Wayland/X11 clipboard.

### Modified Capabilities
- `tool-palette`: Update UI to include export actions group.
