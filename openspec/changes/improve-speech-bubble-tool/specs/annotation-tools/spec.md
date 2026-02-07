## MODIFIED Requirements

### Requirement: Speech bubble draw interaction
The speech bubble tool SHALL create annotations by click-and-drag, starting at zero size from the click point and expanding as the user drags, matching the behavior of the rectangle and ellipse tools. The pointer SHALL auto-position below the bubble center during creation.

#### Scenario: Click and drag to create bubble
- **WHEN** the user clicks and drags with the speech bubble tool
- **THEN** the bubble SHALL expand from the click origin, and the pointer SHALL auto-position below the bubble center with an offset proportional to the bubble height

#### Scenario: Click without drag creates default-sized bubble
- **WHEN** the user clicks without dragging (or drags less than 60x30)
- **THEN** a minimum-sized bubble (60x30) SHALL be created at the click point with a default pointer position, and text editing SHALL start automatically

#### Scenario: Pointer updates during creation drag
- **WHEN** the user is dragging to create a speech bubble
- **THEN** the pointer position SHALL update every frame to stay centered below the bubble
