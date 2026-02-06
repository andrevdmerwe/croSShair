# UX Improvements and New Tools

## Problem Description

Users have requested several enhancements to improve the workflow and add new annotation capabilities:

1. **Focus Handling**: When capturing via hotkey, the app should not steal focus from the current window.
2. **Tool State Reset**: When the last tab is closed, the tool settings (selected tool, color, line width) should reset to defaults.
3. **Highlight Tool**: A rectangle highlight tool to draw attention to areas without obscuring content.
4. **Speech Bubble Tool**: A speech bubble annotation for adding callouts and comments.

## Proposed Solution

Implement the four features as follows:

1. **Focus Handling**: Modify window creation to prevent focus when launched with `capture-region` argument. Use Tauri's window focus APIs.
2. **Tool State Reset**: Add a reset function to `tools.svelte.ts` and call it when `tabs` becomes empty.
3. **Highlight Tool**: New annotation type `highlight` - a semi-transparent filled rectangle with configurable color and opacity.
4. **Speech Bubble Tool**: New annotation type `speechBubble` - combines a rounded rectangle with a triangular pointer and text.

## Acceptance Criteria

- [ ] Capture via hotkey does not steal focus from the active window.
- [ ] Closing the last tab resets tool, color, and line width to defaults.
- [ ] Highlight tool renders a semi-transparent rectangle.
- [ ] Speech bubble tool renders a rounded rectangle with a pointer tail and text.
- [ ] All existing annotation tools continue to work correctly.
