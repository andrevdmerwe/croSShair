# Capability Spec: Speech Bubble Tool

## Requirements

### REQ-BUBBLE-001: Render Speech Bubble
- **WHEN** the user draws a speech bubble annotation
- **THEN** a rounded rectangle with a triangular pointer MUST be rendered

### REQ-BUBBLE-002: Text Content
- The speech bubble MUST support text content inside the rounded rectangle.
- Text editing uses the same input mechanism as the Text tool.

### REQ-BUBBLE-003: Pointer Position
- The triangular pointer indicates the "speaker" or area of focus.
- Pointer position (`pointerX`, `pointerY`) is stored relative to the bubble.
- Pointer is draggable independently of the bubble body.

### REQ-BUBBLE-004: Styling
- Default fill: White (`#FFFFFF`)
- Default stroke: Black (`#000000`)
- Default text color: Black (`#000000`)
- Rounded corners (radius ~10px)

### REQ-BUBBLE-005: Hit Testing
- Speech bubble annotations MUST be selectable, movable, resizable, and deletable.

## Visual Specification
- Rounded rectangle body with text centered.
- Triangular pointer extending from the bottom edge toward `pointerX`, `pointerY`.
