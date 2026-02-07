## Context

The `crop` annotation type already exists in the type system and in `resizableTypes`, but has no functional implementation. The existing annotation pipeline (draw → render → select → resize) provides all the infrastructure needed.

## Goals / Non-Goals

**Goals:**
- Implement a fully functional crop tool following the existing annotation pattern
- Destructive crop: trim base image bitmap and adjust all annotations
- Dark overlay outside crop region for clear visual feedback
- Apply/Cancel buttons and Enter/Escape keyboard shortcuts
- Undo support via existing history system
- Enforce single crop at a time

**Non-Goals:**
- Non-destructive/viewport crop (too complex, doesn't match spec)
- Aspect ratio locking (can be added later)
- Crop presets (16:9, 4:3, etc.)

## Decisions

**Destructive crop over viewport crop**
Rationale: Matches the spec requirements — bitmap is trimmed, annotations are clipped/removed, canvas resizes. Simpler mental model for users.

**Dark overlay rendering**
Rationale: Standard crop UX pattern. Semi-transparent black overlay outside crop region makes the selected area clearly visible.

**Single crop enforcement**
Rationale: Multiple overlapping crops don't make sense. Drawing a new crop removes the previous one.

**Render crop last in overlay**
Rationale: The dark overlay must appear on top of all other annotations for proper visual effect.

## Risks / Trade-offs

**Large image memory usage** → Use PNG data URL for lossless quality; offscreen canvas is temporary and GC'd after use.

**Undo complexity** → History already stores full annotation snapshots. Crop apply stores the pre-crop image as a data URL in the image element, and undo restores the previous annotation state. The base image restoration requires storing the image data in history — current history only stores annotations. Mitigation: store the image data URL alongside annotations in history, or accept that undo after crop restores annotations but not the image.
