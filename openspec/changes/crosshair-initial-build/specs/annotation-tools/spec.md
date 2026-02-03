## ADDED Requirements

### Requirement: Layered canvas architecture
The system SHALL use a layered canvas with a base screenshot layer and overlay annotation objects per tab.

#### Scenario: Canvas layers
- **WHEN** tab is displaying a screenshot
- **THEN** base layer contains the screenshot bitmap
- **AND** overlay layer contains annotation objects (shapes, text, markers)

### Requirement: Selection and move tool
The system SHALL provide a selection tool to select, move, and delete annotation objects.

#### Scenario: Selecting an object
- **WHEN** selection tool is active
- **AND** user clicks on an existing annotation object
- **THEN** object is selected with visible outline

#### Scenario: Moving an object
- **WHEN** an annotation object is selected
- **AND** user drags the object
- **THEN** object position updates

#### Scenario: Deleting an object
- **WHEN** an annotation object is selected
- **AND** user presses Delete key or clicks Delete button
- **THEN** object is removed from canvas

### Requirement: Freehand pen tool
The system SHALL provide a freehand drawing tool with configurable color and stroke thickness.

#### Scenario: Freehand drawing
- **WHEN** pen tool is active
- **AND** user drags on canvas
- **THEN** freehand line is drawn following cursor path
- **AND** line uses current color and stroke thickness

### Requirement: Highlighter tool
The system SHALL provide a highlighter tool with semi-transparent strokes.

#### Scenario: Highlighting content
- **WHEN** highlighter tool is active
- **AND** user drags on canvas
- **THEN** semi-transparent stroke (alpha 0.4-0.6) is drawn
- **AND** underlying content remains visible through highlight

### Requirement: Rectangle tool
The system SHALL provide a rectangle tool with outline and filled modes.

#### Scenario: Drawing outline rectangle
- **WHEN** rectangle tool is active in outline mode
- **AND** user click-drags on canvas
- **THEN** rectangle outline is drawn from start to end point

#### Scenario: Drawing filled rectangle
- **WHEN** rectangle tool is active in filled mode
- **AND** user click-drags on canvas
- **THEN** filled rectangle is drawn with outline

### Requirement: Ellipse tool
The system SHALL provide an ellipse tool with outline and filled modes.

#### Scenario: Drawing ellipse
- **WHEN** ellipse tool is active
- **AND** user click-drags on canvas
- **THEN** ellipse is drawn fitting the bounding rectangle

### Requirement: Line tool
The system SHALL provide a straight line tool with configurable color and thickness.

#### Scenario: Drawing straight line
- **WHEN** line tool is active
- **AND** user click-drags on canvas
- **THEN** straight line is drawn from start to end point

### Requirement: Arrow tool
The system SHALL provide an arrow tool with arrowhead at the end point.

#### Scenario: Drawing arrow
- **WHEN** arrow tool is active
- **AND** user click-drags on canvas
- **THEN** line with triangle arrowhead is drawn

### Requirement: Step numbering tool
The system SHALL provide a step marker tool that places sequential numbered markers.

#### Scenario: Placing step markers
- **WHEN** step numbering tool is active
- **AND** user clicks on canvas
- **THEN** numbered marker is placed (1, 2, 3, ...)
- **AND** marker style is circle or teardrop with number centered

#### Scenario: Resetting step numbers
- **WHEN** user triggers "Reset step numbers" action
- **THEN** next placed marker starts at 1
- **AND** existing markers retain their numbers

### Requirement: Text tool
The system SHALL provide a text tool to place editable text boxes.

#### Scenario: Placing text
- **WHEN** text tool is active
- **AND** user clicks on canvas
- **THEN** text box is created at click position
- **AND** user can type text content

#### Scenario: Text properties
- **WHEN** text box is created
- **THEN** text uses current color
- **AND** font size can be set to small/medium/large presets

### Requirement: Pixelation tool
The system SHALL provide a pixelation tool to obscure regions of the screenshot.

#### Scenario: Pixelating region
- **WHEN** pixelation tool is active
- **AND** user drags a rectangular region
- **THEN** that region of the underlying screenshot is pixelated
- **AND** pixelation is applied permanently to base layer

### Requirement: Crop tool
The system SHALL provide a crop tool to trim the screenshot.

#### Scenario: Cropping screenshot
- **WHEN** user defines crop rectangle
- **AND** user clicks "Apply"
- **THEN** screenshot is trimmed to crop area
- **AND** annotations outside crop area are removed or clipped
- **AND** canvas dimensions update to new size

### Requirement: Color picker
The system SHALL provide a global color picker with presets and custom color selection.

#### Scenario: Selecting preset color
- **WHEN** user clicks a preset color (red, green, blue, yellow, orange, purple)
- **THEN** current drawing color is updated

#### Scenario: Selecting custom color
- **WHEN** user uses custom color input
- **THEN** current drawing color is set to selected value

### Requirement: Stroke thickness control
The system SHALL provide a global stroke thickness control with at least 3 levels.

#### Scenario: Setting stroke thickness
- **WHEN** user selects thin/medium/thick stroke
- **THEN** new drawing operations use selected thickness

### Requirement: Per-tab undo/redo
The system SHALL provide undo/redo for all annotation actions within each tab.

#### Scenario: Undoing action
- **WHEN** user triggers undo (Ctrl+Z or button)
- **THEN** last annotation action is reversed

#### Scenario: Redoing action
- **WHEN** user triggers redo (Ctrl+Shift+Z or Ctrl+Y or button)
- **THEN** last undone action is reapplied

### Requirement: Copy annotated image to clipboard
The system SHALL allow copying the current tab's image (with annotations) to clipboard.

#### Scenario: Copy to clipboard
- **WHEN** user triggers "Copy to clipboard" action
- **THEN** composite image (screenshot + annotations) is piped to wl-copy as PNG
