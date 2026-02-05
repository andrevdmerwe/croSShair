# Tool Palette (Delta)

## Summary
Modifications to the `ToolPalette` to include output and export controls.

## Requirements

### New UI Elements
- **Output Section**: A new section in the palette (typically at the top or very bottom) containing output actions.
- **Save Button**: An icon button (e.g., Floppy Disk) triggering the "Save As" flow.
- **Copy Button**: An icon button (e.g., Clipboard) triggering the "Copy to Clipboard" flow.

### Behavior
- **Tool Independence**: These actions are NOT tools; they do not change the active drawing mode. They are immediate-action buttons.

## Dependencies
- Requires `image-export` logic.
- Requires `clipboard-integration` logic.
