## Context

The speech bubble tool renders using a 3-step process: draw ellipse with fill+stroke, fill a pointer triangle over the ellipse stroke, then stroke the pointer edges. This creates a visible border discontinuity at the connection point. The creation interaction uses a fixed 80x40 default size instead of expanding from the click point, and the pointer position is never updated during drag.

## Goals / Non-Goals

**Goals:**
- Seamless visual connection between bubble body and pointer (single unified path)
- Ellipse body shape (preserved from original)
- Draw interaction matching rectangle/ellipse tools (expand from click point)
- Hit testing that includes the pointer triangle area
- Selection bounding box that encompasses the pointer

**Non-Goals:**
- Changing the pointer drag behavior after creation (decoupled movement is intentional)
- Adding new data properties to the Annotation type
- Modifying text editing behavior within the bubble

## Decisions

**Ellipse body with unified path**
The body remains an ellipse. The pointer is integrated into the ellipse path by calculating the angles where the pointer base meets the ellipse curve, then drawing a partial ellipse arc (from left base around the top to right base) followed by lines to the pointer tip. One `fill()` and one `stroke()` call eliminates the border gap. Alternative considered: rounded rectangle body — rejected to preserve the original visual style.

**Single composite canvas path**
The entire shape (body + pointer) is drawn as one continuous path using `ctx.ellipse()` with partial arc angles. The pointer base angles are computed via `Math.acos(pointerBaseHalf / radiusX)`. The arc skips the bottom segment where the pointer connects, and `lineTo` + `closePath` draw the pointer triangle. Alternative considered: using `globalCompositeOperation` to merge separate draws — rejected as more complex and fragile.

**Minimum size on click-without-drag**
If the user clicks without dragging (bubble smaller than 60x30), snap to a default 60x30 size with pointer below center. This prevents invisible zero-size bubbles while preserving the click-to-create flow.

## Risks / Trade-offs

**[Risk] Pointer base angle precision** → For very small bubbles where `pointerBaseHalf >= radiusX`, the `Math.acos` calculation is invalid. Mitigated by falling back to a simple full ellipse (no integrated pointer) in that case.

**[Risk] Existing annotations render identically** → The ellipse shape is preserved, so existing annotations look the same. Only the pointer connection quality improves. No migration needed.
