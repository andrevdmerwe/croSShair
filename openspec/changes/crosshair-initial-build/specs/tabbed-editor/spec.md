## ADDED Requirements

### Requirement: Browser-style tab bar
The system SHALL display a horizontal tab bar at the top of the window where each tab represents one screenshot editing session.

#### Scenario: Tab bar display
- **WHEN** application window is visible
- **THEN** horizontal tab bar is shown at top
- **AND** each tab shows a label and close button
- **AND** active tab is visually highlighted

### Requirement: New tab on capture
The system SHALL open each captured screenshot in a new tab for editing.

#### Scenario: Screenshot opens in new tab
- **WHEN** screenshot is captured successfully
- **THEN** new tab is created with the screenshot
- **AND** tab is labeled with timestamp (e.g., "2026-02-03 20:58:12") or shot number

### Requirement: Tab close behavior
The system SHALL remove the screenshot and annotations from memory when a tab is closed, without deleting the saved file.

#### Scenario: Closing a tab
- **WHEN** user clicks tab close button
- **THEN** tab is removed from tab bar
- **AND** screenshot and annotations are removed from memory
- **AND** original saved file on disk is NOT deleted

### Requirement: Empty state display
The system SHALL show a helpful empty state when no tabs are open.

#### Scenario: No tabs open
- **WHEN** no screenshot tabs are open
- **THEN** centered message shows "No screenshots yet"
- **AND** "Take region screenshot" button is displayed
- **AND** clicking button triggers capture pipeline

### Requirement: Dirty state tracking
The system SHALL track unsaved changes per tab and prompt before losing changes.

#### Scenario: Closing dirty tab
- **WHEN** user attempts to close a tab with unsaved changes
- **THEN** dialog shows with "Save", "Discard", "Cancel" options

#### Scenario: Exiting with dirty tabs
- **WHEN** user attempts to exit application with any dirty tabs
- **THEN** dialog shows prompting to save changes
