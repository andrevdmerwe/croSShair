## ADDED Requirements

### Requirement: Configurable Storage Path
The system SHALL allow users to select a default directory for saving images.

#### Scenario: Select Storage Folder
- **WHEN** user clicks "Select Folder" in Settings
- **AND** chooses a directory via the system dialog
- **THEN** the application stores this path as the default save location

### Requirement: Save to Default Path
Capture operations SHALL default to the configured storage path.

#### Scenario: Save Image
- **WHEN** user triggers a capture/save
- **THEN** the file is saved to the Configured Storage Path without prompting (or defaults the prompt to that path, depending on implementation)
