## ADDED Requirements

### Requirement: Initialize Tools with Defaults
Tools SHALL initialize their properties (color, width) from the configured defaults when selected, unless carrying over state from a previous tool interaction (if designed to do so).

#### Scenario: Reset to Default
- **WHEN** application starts
- **THEN** the active tool uses the Default Color and Default Line Width configured in Settings

### Requirement: Pixelate Tool Density Default
The Pixelate tool SHALL uses the configured Default Pixel Density.

#### Scenario: Pixelate Default
- **WHEN** user selects Pixelate tool
- **THEN** the block size slider is set to the configured Default Pixel Density
