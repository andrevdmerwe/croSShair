## Why

Users currently cannot persist their preferences or customize the application defaults (like stroke width, colors, or storage location). This leads to repetitive setup every time they launch simple-crosshair. A settings menu is needed to provide a centralized place for these persistent configurations.

## What Changes

- **New Settings Interface**: A new modal or dedicated view for managing application settings.
- **Default Configuration**:
    - **Default Tool**: Choose which tool is active on startup.
    - **Default Line Width**: Set the initial stroke width for drawing tools.
    - **Default Color**: Set the initial active color.
    - **Default Pixel Density**: Set the default block size for the pixelate tool.
- **Toolbar Customization**: Allow users to customize the color palette available in the toolbar.
- **Storage Configuration**: Allow users to select the default folder for saving screenshots/images.

## Capabilities

### New Capabilities
- `settings-management`: Core capability to save, load, and manage persistent user preferences (defaults, paths).
- `palette-customization`: Capability to modify the set of colors available in the main toolbar.

### Modified Capabilities
- `tool-usage`: Will need to respect the configured defaults on initialization.
- `screenshot-storage`: Will need to use the configured default storage folder.

## Impact

- **Frontend**:
    - New `Settings` component/modal.
    - Updates to `ToolPalette` to render dynamic colors and read defaults.
    - Updates to `Canvas` or Tool Stores to initialize with saved defaults.
- **State Management**:
    - New `SettingsStore` (persisted to localStorage or Tauri store).
- **Backend (Tauri)**:
    - May require file dialogs for folder selection (already supported).
