# Capability Spec: Highlight Tool

## Requirements

### REQ-HIGHLIGHT-001: Render Highlight
- **WHEN** the user draws a highlight annotation
- **THEN** a semi-transparent filled rectangle MUST be rendered

### REQ-HIGHLIGHT-002: Highlight Properties
- Default color: Yellow (`#FFFF00`)
- Default opacity: 30%
- Resizable and movable like other rectangle-based tools

### REQ-HIGHLIGHT-003: Highlight Hit Testing
- Highlight annotations MUST be selectable, movable, resizable, and deletable.

## Visual Specification
- Filled rectangle with no stroke.
- Semi-transparent fill to allow underlying content to show through.
