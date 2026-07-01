## ADDED Requirements

### Requirement: Zoom Toolbar Controls
The canvas SHALL display zoom controls (zoom out, current zoom percentage, zoom in, reset-to-fit) that are visible whenever an image is loaded, and SHALL update the displayed percentage whenever the zoom level changes.

#### Scenario: Zoom controls visible with image loaded
- **WHEN** a screenshot is loaded into the active tab
- **THEN** the canvas displays zoom-out, zoom-in, reset-to-fit controls and the current zoom percentage

#### Scenario: Clicking zoom in increases zoom
- **WHEN** the user clicks the zoom-in control
- **THEN** the image scale increases by a fixed step and the displayed percentage updates to match

#### Scenario: Clicking zoom out decreases zoom
- **WHEN** the user clicks the zoom-out control
- **THEN** the image scale decreases by a fixed step and the displayed percentage updates to match

### Requirement: Keyboard Zoom Shortcuts
The application SHALL support `Ctrl +` (or `Ctrl =`) to zoom in, `Ctrl -` to zoom out, and `Ctrl 0` to reset to fit-to-window, matching the existing pattern used for undo/redo shortcuts, and SHALL prevent the WebView's native zoom behavior when handling these keys.

#### Scenario: Zoom in via keyboard
- **WHEN** the user presses `Ctrl` and `+` (or `=`) while the canvas is focused and not editing text
- **THEN** the image scale increases by a fixed step, the default browser/WebView zoom action does not occur, and the displayed percentage updates

#### Scenario: Zoom out via keyboard
- **WHEN** the user presses `Ctrl -` while the canvas is focused and not editing text
- **THEN** the image scale decreases by a fixed step and the displayed percentage updates

#### Scenario: Reset to fit via keyboard
- **WHEN** the user presses `Ctrl 0`
- **THEN** the image scale returns to the fit-to-window value and the canvas re-enters fit-to-window mode

### Requirement: Cursor-Centered Wheel Zoom
The canvas SHALL zoom in/out when the user scrolls the mouse wheel while holding `Ctrl`, keeping the point of the image under the cursor stationary on screen, and SHALL NOT pan or zoom on a plain (non-Ctrl) wheel scroll — plain scroll continues to pan the canvas via native scrolling.

#### Scenario: Ctrl+wheel zooms toward cursor
- **WHEN** the user holds `Ctrl` and scrolls the mouse wheel while the cursor is over a point on the image
- **THEN** the zoom level changes by an amount proportional to the scroll delta, clamped to the min/max zoom bounds, and the same image point remains under the cursor after the zoom

#### Scenario: Plain wheel scroll still pans
- **WHEN** the user scrolls the mouse wheel without holding `Ctrl` over a canvas that is larger than its container
- **THEN** the canvas container scrolls (pans) as before and the zoom level does not change

### Requirement: Zoom Bounds
The application SHALL clamp the zoom level to a minimum of 10% and a maximum of 500% of the image's natural size, regardless of which zoom control (button, keyboard, or wheel) triggered the change.

#### Scenario: Cannot zoom below minimum
- **WHEN** the user repeatedly zooms out at or near the minimum zoom level
- **THEN** the zoom level stops decreasing once it reaches 10% and the zoom-out control does not zoom below that

#### Scenario: Cannot zoom above maximum
- **WHEN** the user repeatedly zooms in at or near the maximum zoom level
- **THEN** the zoom level stops increasing once it reaches 500% and the zoom-in control does not zoom above that

### Requirement: Reset to Fit
The canvas SHALL provide a reset-to-fit control that recomputes the zoom level to fit the current image within the container (the same calculation used when the image is first loaded), and re-enables automatic re-fitting on subsequent container resizes until the user manually zooms again.

#### Scenario: Reset after manual zoom
- **WHEN** the user has manually zoomed in or out and then activates reset-to-fit
- **THEN** the zoom level returns to the fit-to-window value for the current container size and image dimensions

#### Scenario: Fit-to-window tracks window resize until manual zoom
- **WHEN** the user has not manually zoomed since the image loaded (or since the last reset-to-fit) and the container is resized
- **THEN** the zoom level recomputes to fit the new container size, exactly as it does today

#### Scenario: Manual zoom stops auto-refit on resize
- **WHEN** the user has manually zoomed (via button, keyboard, or wheel) and the container is subsequently resized
- **THEN** the zoom level does NOT change due to the resize; only the manual zoom controls or reset-to-fit change it

### Requirement: Zoom-Independent Annotation Interaction
Annotation rendering, hit-testing, resize-handle sizing, and the in-progress text-edit overlay SHALL remain visually and functionally correct at any supported zoom level, including levels above 100%.

#### Scenario: Annotate at high zoom
- **WHEN** the user zooms in above 100% and draws, selects, or resizes an annotation
- **THEN** the annotation is created/selected/resized at the correct image-space coordinates, and on-screen handle sizes and hit-test thresholds remain a constant visual size regardless of zoom level

#### Scenario: Text editing overlay follows zoom
- **WHEN** the user is editing a text annotation while zoomed to a level other than 100%
- **THEN** the floating text input's position, width, height, and font size are scaled to match the current zoom level
