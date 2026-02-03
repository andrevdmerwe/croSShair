## ADDED Requirements

### Requirement: XDG-compliant config storage
The system SHALL store configuration in a JSON file at `$XDG_CONFIG_HOME/croSShair/config.json` or `~/.config/croSShair/config.json`.

#### Scenario: Config file location with XDG_CONFIG_HOME set
- **WHEN** application starts
- **AND** XDG_CONFIG_HOME environment variable is set
- **THEN** config is loaded from/saved to `$XDG_CONFIG_HOME/croSShair/config.json`

#### Scenario: Config file location without XDG_CONFIG_HOME
- **WHEN** application starts
- **AND** XDG_CONFIG_HOME is not set
- **THEN** config is loaded from/saved to `~/.config/croSShair/config.json`

### Requirement: Screenshot directory setting
The system SHALL allow configuration of the screenshot save directory with a default of `~/Pictures/Screenshots`.

#### Scenario: Default screenshot directory
- **WHEN** no screenshotDirectory is configured
- **THEN** screenshots are saved to `~/Pictures/Screenshots`

#### Scenario: Custom screenshot directory
- **WHEN** screenshotDirectory is configured to a custom path
- **THEN** screenshots are saved to that path

### Requirement: Copy after capture setting
The system SHALL provide a boolean setting to enable/disable automatic clipboard copy on capture.

#### Scenario: Copy after capture setting toggle
- **WHEN** user toggles "copy after capture" setting
- **THEN** setting value is persisted
- **AND** subsequent captures respect the new value

### Requirement: Default color settings
The system SHALL persist default annotation colors.

#### Scenario: Default color persistence
- **WHEN** user changes default color in settings
- **THEN** color is persisted to config
- **AND** new annotation operations use the saved color

### Requirement: Default stroke width setting
The system SHALL persist default stroke width (thin/medium/thick).

#### Scenario: Default stroke width persistence
- **WHEN** user changes default stroke width
- **THEN** value is persisted to config
- **AND** new drawing operations use the saved width

### Requirement: Settings UI
The system SHALL provide a settings screen with controls for all configuration options.

#### Scenario: Directory picker
- **WHEN** user opens settings
- **THEN** directory picker is available for screenshot save location

#### Scenario: Clipboard toggle
- **WHEN** user opens settings
- **THEN** checkbox for "Copy captured screenshot to clipboard automatically" is available

#### Scenario: Color configuration
- **WHEN** user opens settings
- **THEN** default primary color selector is available

#### Scenario: Stroke thickness selector
- **WHEN** user opens settings
- **THEN** default stroke thickness selector is available

### Requirement: Global shortcut guidance
The system SHALL display guidance for configuring global shortcuts in Niri.

#### Scenario: Shortcut guidance display
- **WHEN** user views settings
- **THEN** section explains global shortcuts are managed by compositor
- **AND** CLI command `croSShair capture-region` is displayed
- **AND** example Niri binds configuration is shown
