## ADDED Requirements

### Requirement: Unified shape rendering
The speech bubble SHALL be rendered as a single continuous canvas path combining a rounded rectangle body with an integrated pointer triangle. The path SHALL use one `fill()` and one `stroke()` call to ensure no border discontinuity at the pointer connection.

#### Scenario: Bubble renders with seamless pointer connection
- **WHEN** a speech bubble annotation is rendered with a pointer
- **THEN** the body and pointer SHALL appear as a single unified shape with a continuous border

#### Scenario: Bubble renders without pointer
- **WHEN** a speech bubble annotation has no pointer data
- **THEN** the body SHALL render as a rounded rectangle without a pointer

### Requirement: Rounded rectangle body shape
The speech bubble body SHALL use a rounded rectangle shape with corner radius `Math.min(12, width * 0.15, height * 0.15)`.

#### Scenario: Corner radius scales with size
- **WHEN** a speech bubble is drawn at various sizes
- **THEN** the corner radius SHALL be proportional to the bubble dimensions, capped at 12px

### Requirement: Pointer base clamping
The pointer base connection point SHALL be clamped to remain within the bottom edge of the rounded rectangle, between the two bottom corner arcs.

#### Scenario: Pointer dragged to extreme side
- **WHEN** the pointer tip is dragged far to the left or right
- **THEN** the pointer base SHALL stay within the bottom edge boundaries and not overlap corner arcs

### Requirement: Pointer triangle hit testing
Clicking on the pointer triangle area SHALL select the speech bubble annotation.

#### Scenario: Click on pointer selects annotation
- **WHEN** the user clicks within the pointer triangle area (below the bubble body)
- **THEN** the annotation SHALL be selected

### Requirement: Bounding box includes pointer
The selection bounding box SHALL expand to encompass both the bubble body and the pointer tip position.

#### Scenario: Selection outline wraps pointer
- **WHEN** a speech bubble with a pointer is selected
- **THEN** the dashed selection outline SHALL surround both the body and the pointer tip
