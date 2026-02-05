# Post-Launch Feature Roadmap

Based on the analysis of the current codebase (`croSShair` v0.1.0), the foundational capture and annotation tools are in place. The next phase should focus on **Output**, **Sharing**, and **Text Annotation**.

## Phase 1: Output & Sharing (Critical)
The application currently captures an image and allows non-destructive annotation, but there is no way to save the *result* or copy the *annotated* image.

- [ ] **Export/Save Image**
    - Create a mechanism to "burn" the SVG/Canvas annotations onto the base image.
    - Add a "Save As..." option to export the final composition.
    - Overwrite original file option.
- [ ] **Copy Edited Image to Clipboard**
    - Add logic to flatten the image in memory and write the blob to the clipboard.
    - Add a specialized button for "Copy Result".

## Phase 2: Essential Annotation Tools
- [ ] **Text Tool**
    - Implement a text input annotation type.
    - Support font size and color handling.
    - Handling multi-line text (optional but good).
- [ ] **Crop Tool**
    - Allow post-capture cropping of the image.
    - This is distinct from resizing the window; it actually crops the image data.

## Phase 3: UX & Productivity
- [ ] **Undo/Redo UI**
    - Expose the existing history functionality via Toolbar buttons (Undo/Redo icons).
- [ ] **Keyboard Shortcuts Help**
    - A small modal or tooltip list showing available hotkeys (Ctrl+Z, Ctrl+Y, Ctrl+S, etc.).
- [ ] **Blur Tool**
    - Complement the "Pixelate" tool with a Gaussian Blur option for smoother redaction.

## Technical Tasks (Refactoring)
- [ ] **Optimized Rendering**: Ensure large images don't cause performance lag with many annotations.
- [ ] **Error Handling**: hardening the backend integration for clipboard failures on different Wayland compositors.