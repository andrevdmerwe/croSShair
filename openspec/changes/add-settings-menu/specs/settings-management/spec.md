## ADDED Requirements

### Requirement: Persist User Preferences
The system SHALL persist user preferences across application sessions using local storage.

#### Scenario: Save Settings
- **WHEN** user changes a setting in the Settings Menu
- **THEN** the setting is immediately saved to persistent storage

#### Scenario: Load Settings on Startup
- **WHEN** the application launches
- **THEN** the system loads the saved settings
- **AND** applies them to the application state

### Requirement: Default Tool Configuration
The system SHALL allow users to configure the default tool active on startup.

#### Scenario: Configure Default Tool
- **WHEN** user selects "Rectangle" as the default tool in Settings
- **AND** restarts the application
- **THEN** the "Rectangle" tool is active upon launch

### Requirement: Default Stroke Properties
The system SHALL allow users to configure the default stroke width and pixel density.

#### Scenario: Configure Default Stroke Width
- **WHEN** user sets default stroke width to 5px
- **THEN** new drawing tools initialize with 5px width (unless overridden)
