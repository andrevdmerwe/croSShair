import type { Annotation } from '../types';

/** Threshold distance (in pixels) for line/path hit detection */
const HIT_THRESHOLD = 8;

/**
 * Test if a point (x, y) hits an annotation
 * @returns true if the point is within/on the annotation
 */
export function hitTestAnnotation(ann: Annotation, x: number, y: number): boolean {
    switch (ann.type) {
        case 'rectangle':
            return hitTestRectangle(ann, x, y);
        case 'ellipse':
            return hitTestEllipse(ann, x, y);
        case 'pen':
        case 'highlighter':
            return hitTestPath(ann, x, y, HIT_THRESHOLD + ann.strokeWidth / 2);
        case 'line':
        case 'arrow':
            return hitTestLine(ann, x, y, HIT_THRESHOLD + ann.strokeWidth / 2);
        case 'text':
            return hitTestRectangle(ann, x, y); // Text uses bounding box
        case 'step':
            return hitTestStep(ann, x, y);
        default:
            return false;
    }
}

/**
 * Point-in-rectangle test (including stroke width)
 */
function hitTestRectangle(ann: Annotation, x: number, y: number): boolean {
    const halfStroke = ann.strokeWidth / 2;
    const left = Math.min(ann.x, ann.x + (ann.width ?? 0)) - halfStroke;
    const right = Math.max(ann.x, ann.x + (ann.width ?? 0)) + halfStroke;
    const top = Math.min(ann.y, ann.y + (ann.height ?? 0)) - halfStroke;
    const bottom = Math.max(ann.y, ann.y + (ann.height ?? 0)) + halfStroke;

    return x >= left && x <= right && y >= top && y <= bottom;
}

/**
 * Point-in-ellipse test
 */
function hitTestEllipse(ann: Annotation, x: number, y: number): boolean {
    const w = ann.width ?? 0;
    const h = ann.height ?? 0;
    const cx = ann.x + w / 2;
    const cy = ann.y + h / 2;
    const rx = Math.abs(w / 2) + ann.strokeWidth / 2;
    const ry = Math.abs(h / 2) + ann.strokeWidth / 2;

    if (rx === 0 || ry === 0) return false;

    // Ellipse equation: ((x-cx)/rx)^2 + ((y-cy)/ry)^2 <= 1
    const dx = (x - cx) / rx;
    const dy = (y - cy) / ry;
    return (dx * dx + dy * dy) <= 1;
}

/**
 * Proximity test for freehand paths (pen/highlighter)
 */
function hitTestPath(ann: Annotation, x: number, y: number, threshold: number): boolean {
    const points = ann.data?.points as { x: number; y: number }[] | undefined;
    if (!points || points.length === 0) return false;

    // Check distance to each segment
    for (let i = 0; i < points.length - 1; i++) {
        const dist = pointToSegmentDistance(x, y, points[i].x, points[i].y, points[i + 1].x, points[i + 1].y);
        if (dist <= threshold) return true;
    }

    // Also check first point for single-point paths
    if (points.length === 1) {
        const dx = x - points[0].x;
        const dy = y - points[0].y;
        return Math.sqrt(dx * dx + dy * dy) <= threshold;
    }

    return false;
}

/**
 * Proximity test for straight lines and arrows
 */
function hitTestLine(ann: Annotation, x: number, y: number, threshold: number): boolean {
    const endX = ann.data?.endX ?? (ann.x + (ann.width ?? 0));
    const endY = ann.data?.endY ?? (ann.y + (ann.height ?? 0));

    const dist = pointToSegmentDistance(x, y, ann.x, ann.y, endX, endY);
    return dist <= threshold;
}

/**
 * Hit test for step markers (circular shape)
 */
function hitTestStep(ann: Annotation, x: number, y: number): boolean {
    // Step markers are typically 24-30px diameter circles
    const radius = 15 + ann.strokeWidth / 2;
    const dx = x - ann.x;
    const dy = y - ann.y;
    return (dx * dx + dy * dy) <= radius * radius;
}

/**
 * Calculate the minimum distance from a point (px, py) to a line segment (x1,y1)-(x2,y2)
 */
function pointToSegmentDistance(
    px: number, py: number,
    x1: number, y1: number,
    x2: number, y2: number
): number {
    const dx = x2 - x1;
    const dy = y2 - y1;
    const lengthSquared = dx * dx + dy * dy;

    if (lengthSquared === 0) {
        // Segment is a point
        return Math.sqrt((px - x1) ** 2 + (py - y1) ** 2);
    }

    // Parameter t represents position along segment (0 = start, 1 = end)
    let t = ((px - x1) * dx + (py - y1) * dy) / lengthSquared;
    t = Math.max(0, Math.min(1, t)); // Clamp to segment

    // Find closest point on segment
    const closestX = x1 + t * dx;
    const closestY = y1 + t * dy;

    return Math.sqrt((px - closestX) ** 2 + (py - closestY) ** 2);
}

/**
 * Get the bounding box of an annotation (useful for selection outline)
 */
export function getAnnotationBounds(ann: Annotation): { x: number; y: number; width: number; height: number } {
    switch (ann.type) {
        case 'pen':
        case 'highlighter': {
            const points = ann.data?.points as { x: number; y: number }[] | undefined;
            if (!points || points.length === 0) {
                return { x: ann.x, y: ann.y, width: 0, height: 0 };
            }
            let minX = points[0].x, maxX = points[0].x;
            let minY = points[0].y, maxY = points[0].y;
            for (const p of points) {
                minX = Math.min(minX, p.x);
                maxX = Math.max(maxX, p.x);
                minY = Math.min(minY, p.y);
                maxY = Math.max(maxY, p.y);
            }
            const padding = ann.strokeWidth / 2;
            return {
                x: minX - padding,
                y: minY - padding,
                width: maxX - minX + ann.strokeWidth,
                height: maxY - minY + ann.strokeWidth
            };
        }
        case 'line':
        case 'arrow': {
            const endX = ann.data?.endX ?? (ann.x + (ann.width ?? 0));
            const endY = ann.data?.endY ?? (ann.y + (ann.height ?? 0));
            const padding = ann.strokeWidth / 2;
            return {
                x: Math.min(ann.x, endX) - padding,
                y: Math.min(ann.y, endY) - padding,
                width: Math.abs(endX - ann.x) + ann.strokeWidth,
                height: Math.abs(endY - ann.y) + ann.strokeWidth
            };
        }
        case 'step': {
            const radius = 15;
            return {
                x: ann.x - radius,
                y: ann.y - radius,
                width: radius * 2,
                height: radius * 2
            };
        }
        default: {
            let x = ann.x;
            let y = ann.y;
            let width = ann.width ?? 0;
            let height = ann.height ?? 0;

            // Normalize negative dimensions
            if (width < 0) {
                x += width;
                width = Math.abs(width);
            }
            if (height < 0) {
                y += height;
                height = Math.abs(height);
            }

            return { x, y, width, height };
        }
    }
}
