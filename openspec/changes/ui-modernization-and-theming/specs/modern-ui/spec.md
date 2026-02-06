# Capability Spec: Modern UI

## High Level Requirement
The user interface should be modernized to reflect a premium, consistent aesthetic, utilizing the new Theming Engine. Key areas include the Toolbar, Settings, and Canvas overlays.

## Detailed Requirements

### 1. Toolbar & ToolPalette
-   **Requirement**: The main toolbar should have a "floating" appearance (detached from window edges) with rounded corners.
-   **Requirement**: Use "Glassmorphism" effect (backdrop-filter: blur) for the toolbar background.
-   **Requirement**: Icons should be consistent in size and style. Active tools should have a clear, high-contrast indicator (using `--color-accent`).

### 2. Settings Modal
-   **Requirement**: The Settings modal should match the global theme.
-   **Requirement**: Inputs (text, numbers, checkboxes) should have updated styles (rounded corners, clear focus states using theme variables).
-   **Requirement**: **Theme Selection**: A dropdown menu listing all supported themes.
-   **Requirement**: **Color Customization**: Color pickers to override main theme colors.

### 3. Window & Canvas Interactions
-   **Requirement**: **Frameless Window**: The application window must not have OS decorations (title bar, borders).
-   **Requirement**: **No Window Controls**: The UI should *not* include custom Close/Minimize/Maximize buttons (relying on System Tray/Shortcuts/OS actions).
-   **Requirement**: Canvas overlays (Selection box, Text tool input, Resize handles) must use the global theme variables to ensure visibility on both Light and Dark themes.
    -   *Example*: Selection outline should be `--color-accent` or a high-contrast dynamic color.

### 4. General Polish
-   **Requirement**: Add subtle transitions for hover states and theme switching.
-   **Requirement**: Remove any "browser-default" styles (e.g., default scrollbars, default button bevels).
