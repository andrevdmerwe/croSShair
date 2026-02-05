## ADDED Requirements

### Requirement: Customize Toolbar Palette
The system SHALL allow users to modify the 8 preset colors available in the toolbar.

#### Scenario: Change Palette Color
- **WHEN** user opens Settings
- **AND** clicks on the first color swatch (e.g., Red)
- **AND** selects a new color (e.g., Purple) from the picker
- **THEN** the first color button in the main toolbar updates to Purple
- **AND** all future uses of that palette slot use Purple

#### Scenario: Persist Palette
- **WHEN** user customizes the palette
- **AND** restarts the application
- **THEN** the custom palette is restored
