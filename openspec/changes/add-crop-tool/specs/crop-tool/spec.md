## ADDED Requirements

### Requirement: Crop tool in tool palette
The system SHALL provide a crop tool button in the tool palette that users can select to enter crop mode.

#### Scenario: Selecting crop tool
- **WHEN** user clicks the crop tool button in the tool palette
- **THEN** the current tool is set to "crop"
- **AND** the cursor changes to crosshair

### Requirement: Drawing a crop region
The system SHALL allow users to draw a crop rectangle by clicking and dragging on the canvas.

#### Scenario: Drawing crop rectangle
- **WHEN** user has crop tool selected
- **AND** user clicks and drags on the canvas
- **THEN** a crop rectangle is drawn from the start point to the current cursor position
- **AND** the area outside the crop rectangle is dimmed with a semi-transparent overlay
- **AND** the crop rectangle shows a dashed border

#### Scenario: Only one crop at a time
- **WHEN** user draws a new crop rectangle
- **AND** an existing crop annotation exists
- **THEN** the existing crop annotation is removed before creating the new one

### Requirement: Crop selection and resize
The system SHALL allow users to select and resize the crop rectangle using handles.

#### Scenario: Selecting crop rectangle
- **WHEN** user clicks on the crop rectangle with the select tool
- **THEN** the crop rectangle is selected with resize handles visible

#### Scenario: Resizing crop rectangle
- **WHEN** user drags a resize handle on the crop rectangle
- **THEN** the crop rectangle dimensions update accordingly

### Requirement: Apply crop
The system SHALL provide an "Apply Crop" action that trims the screenshot to the crop area.

#### Scenario: Applying crop
- **WHEN** a crop rectangle exists
- **AND** user clicks the "Apply Crop" button or presses Enter
- **THEN** the screenshot bitmap is trimmed to the crop rectangle area
- **AND** annotations fully outside the crop area are removed
- **AND** annotations partially inside the crop area are clipped
- **AND** annotation coordinates are adjusted to the new origin
- **AND** the canvas dimensions update to the new cropped size
- **AND** the operation is added to history (undoable)

### Requirement: Cancel crop
The system SHALL provide a "Cancel" action to discard the crop without applying.

#### Scenario: Canceling crop
- **WHEN** a crop rectangle exists
- **AND** user clicks "Cancel" or presses Escape
- **THEN** the crop rectangle is removed
- **AND** the original image is unchanged
