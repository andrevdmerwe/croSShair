## ADDED Requirements

### Requirement: Region capture via slurp and grim
The system SHALL capture screenshots by invoking `slurp` for region selection followed by `grim` for image capture.

#### Scenario: Successful region capture
- **WHEN** user initiates region capture
- **THEN** slurp launches for region selection
- **AND** after selection, grim captures the specified region
- **AND** image is saved to configured directory as `YYYYMMDD_HHMMSS.png`

#### Scenario: User cancels region selection
- **WHEN** user presses Escape during slurp region selection
- **THEN** capture is cancelled without error
- **AND** no file is created
- **AND** toast message shows "Capture cancelled"

### Requirement: Automatic save on capture
The system SHALL automatically save captured screenshots to the configured directory using timestamp-based filenames.

#### Scenario: Screenshot saved with timestamp filename
- **WHEN** grim successfully captures a screenshot
- **THEN** file is saved as `YYYYMMDD_HHMMSS.png` (24-hour local time)
- **AND** file is saved to the configured screenshot directory

### Requirement: Optional clipboard copy on capture
The system SHALL optionally copy captured screenshots to the Wayland clipboard using wl-copy when "copy after capture" setting is enabled.

#### Scenario: Copy to clipboard enabled
- **WHEN** screenshot is captured
- **AND** "copy after capture" setting is enabled
- **THEN** screenshot is piped to wl-copy with image/png MIME type

#### Scenario: Copy to clipboard disabled
- **WHEN** screenshot is captured
- **AND** "copy after capture" setting is disabled
- **THEN** screenshot is only saved to file, not copied to clipboard

### Requirement: Dependency validation on startup
The system SHALL check for required CLI tools (grim, slurp, wl-copy) on startup and display guidance if any are missing.

#### Scenario: All dependencies present
- **WHEN** application starts
- **AND** grim, slurp, and wl-copy are in PATH
- **THEN** application launches normally

#### Scenario: Missing dependencies
- **WHEN** application starts
- **AND** one or more of grim, slurp, wl-copy are not in PATH
- **THEN** warning banner shows with installation command suggestion
- **AND** application remains usable for non-capture functionality

### Requirement: Capture feedback
The system SHALL provide visual feedback for capture results via toast notifications.

#### Scenario: Successful capture feedback
- **WHEN** screenshot is captured and saved successfully
- **THEN** toast shows "Region captured and saved to <full path>"

#### Scenario: Capture failure feedback
- **WHEN** capture fails (grim error, invalid path, etc.)
- **THEN** toast shows "Capture failed: <reason>"
- **AND** error details are logged
