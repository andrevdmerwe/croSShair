## ADDED Requirements

### Requirement: CLI capture-region command
The system SHALL provide a CLI entry point `croSShair capture-region` for keybinding integration.

#### Scenario: CLI capture invocation
- **WHEN** user runs `croSShair capture-region`
- **THEN** region capture pipeline is triggered
- **AND** screenshot is saved to configured directory

### Requirement: Single instance with new tab delivery
The system SHALL ensure CLI-triggered captures open as new tabs in an existing running instance.

#### Scenario: Capture with app already running
- **WHEN** `croSShair capture-region` is invoked
- **AND** croSShair is already running
- **THEN** new screenshot opens as new tab in existing window
- **AND** no new application instance is launched

#### Scenario: Capture with app not running
- **WHEN** `croSShair capture-region` is invoked
- **AND** croSShair is not running
- **THEN** application starts
- **AND** capture is performed
- **AND** screenshot opens in new tab

### Requirement: Niri keybinding compatibility
The system SHALL be invocable from Niri keybinding configuration.

#### Scenario: Niri spawn integration
- **WHEN** user configures `binds { Mod+Shift+S { spawn "croSShair" "capture-region"; } }` in Niri
- **AND** user presses Mod+Shift+S
- **THEN** croSShair capture-region is executed

### Requirement: CLI error handling
The system SHALL handle CLI invocation errors gracefully.

#### Scenario: CLI with missing dependencies
- **WHEN** `croSShair capture-region` is invoked
- **AND** grim or slurp is not available
- **THEN** error is logged
- **AND** user is notified via system notification or app UI
