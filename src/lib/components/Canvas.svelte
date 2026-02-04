<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import type { TabData, Annotation } from "$lib/types";

    import { invoke } from "@tauri-apps/api/core";
    import { toastStore } from "$lib/stores/toast.svelte";
    import { toolStore } from "$lib/stores/tools.svelte";
    import { tabStore } from "$lib/stores/tabs.svelte";
    import { getAnnotationBounds, hitTestAnnotation } from "$lib/utils/hitTest";

    let { tab } = $props<{ tab: TabData }>();

    let canvasContainer: HTMLDivElement;
    let baseCanvas: HTMLCanvasElement;
    let overlayCanvas: HTMLCanvasElement;
    let baseCtx: CanvasRenderingContext2D | null = null;
    let overlayCtx: CanvasRenderingContext2D | null = null;

    let scale = $state(1);
    let image = new Image();
    let imageLoaded = $state(false);

    // Load image when tab changes
    $effect(() => {
        if (tab.imagePath) {
            loadImage(tab.imagePath);
        }
    });

    // Drawing state
    let isDrawing = $state(false);
    let drawingAnnotation = $state<Annotation | null>(null);
    let dragAnnotationStartX = $state(0);
    let dragAnnotationStartY = $state(0);

    // Resize state
    let isResizing = $state(false);
    let resizeHandle = $state<string | null>(null);
    let originalResizeRect = $state<{
        x: number;
        y: number;
        width: number;
        height: number;
        pointerX?: number;
        pointerY?: number;
    } | null>(null);

    // Re-render when annotations or selection changes
    $effect(() => {
        if (imageLoaded && overlayCtx) {
            // Depend on annotations and selection for reactivity
            const _ = tab.annotations;
            const __ = toolStore.selectedAnnotationId;
            // Also depend on drawingAnnotation (though it's local state, explicit ref helps)
            const ___ = drawingAnnotation;
            const ____ = isResizing; // Re-render when resizing starts/stops
            renderOverlay();
        }
    });

    async function loadImage(src: string) {
        console.log(`Loading image via Base64: ${src}`);

        try {
            const base64Url = await invoke<string>("read_image_base64", {
                filePath: src,
            });
            console.log("Base64 image received from backend");

            image.onload = () => {
                console.log(
                    "Image loaded successfully:",
                    image.width,
                    "x",
                    image.height,
                );
                imageLoaded = true;
                resizeCanvas();
                renderBase();
                renderOverlay();
            };

            image.onerror = (e) => {
                console.error("Failed to render Base64 image:", e);
                toastStore.add("error", `Failed to render image data: ${src}`);
            };

            image.src = base64Url;
        } catch (e) {
            console.error("Failed to load image from backend:", e);
            toastStore.add("error", `Failed to load image file: ${src} (${e})`);
        }
    }

    function resizeCanvas() {
        if (!canvasContainer || !imageLoaded) return;

        // Fit image to container with some padding
        const containerWidth = canvasContainer.clientWidth - 40;
        const containerHeight = canvasContainer.clientHeight - 40;

        const imgRatio = image.width / image.height;
        const containerRatio = containerWidth / containerHeight;

        if (imgRatio > containerRatio) {
            scale = containerWidth / image.width;
        } else {
            scale = containerHeight / image.height;
        }

        // Limit scale to max 1 for initial view (don't upscale pixelated screenshots)
        scale = Math.min(scale, 1);

        const width = image.width * scale;
        const height = image.height * scale;

        // Set canvas dimensions
        [baseCanvas, overlayCanvas].forEach((canvas) => {
            canvas.width = image.width; // Internal resolution matches original image
            canvas.height = image.height;
            canvas.style.width = `${width}px`; // Display size
            canvas.style.height = `${height}px`;
        });

        renderBase();
        renderOverlay();
    }

    function renderBase() {
        if (!baseCtx || !imageLoaded) return;
        baseCtx.clearRect(0, 0, baseCanvas.width, baseCanvas.height);
        baseCtx.drawImage(image, 0, 0);
        // Apply any crop or pixelation to base layer here (destructive)
    }

    function renderOverlay() {
        if (!overlayCtx) return;
        overlayCtx.clearRect(0, 0, overlayCanvas.width, overlayCanvas.height);

        // Render all annotations
        tab.annotations.forEach(renderAnnotation);

        // Render in-progress drawing
        if (drawingAnnotation) {
            renderAnnotation(drawingAnnotation);
        }

        // Render selection outline if an annotation is selected
        if (toolStore.selectedAnnotationId && !isDrawing) {
            const selected = tab.annotations.find(
                (a: Annotation) => a.id === toolStore.selectedAnnotationId,
            );
            if (selected) {
                renderSelectionOutline(selected);
            }
        }
    }

    function renderSelectionOutline(ann: Annotation) {
        if (!overlayCtx) return;

        const bounds = getAnnotationBounds(ann);
        const padding = 4;

        overlayCtx.save();
        overlayCtx.strokeStyle = "#0088ff";
        overlayCtx.lineWidth = 2; // Fixed width for selection
        overlayCtx.setLineDash([6, 4]);
        overlayCtx.strokeRect(
            bounds.x - padding,
            bounds.y - padding,
            bounds.width + padding * 2,
            bounds.height + padding * 2,
        );

        // Draw resize handles
        overlayCtx.setLineDash([]);
        overlayCtx.fillStyle = "#ffffff";
        overlayCtx.strokeStyle = "#0088ff";
        const handleSize = 8 / scale;

        const handles = getResizeHandles(bounds, padding);

        handles.forEach((h) => {
            overlayCtx!.fillRect(
                h.x - handleSize / 2,
                h.y - handleSize / 2,
                handleSize,
                handleSize,
            );
            overlayCtx!.strokeRect(
                h.x - handleSize / 2,
                h.y - handleSize / 2,
                handleSize,
                handleSize,
            );
        });

        // Special handle for Step Pointer
        if (ann.type === "step" && ann.data?.pointerX !== undefined) {
            overlayCtx.fillStyle = "#ffff00"; // distinct color for pointer handle
            overlayCtx.fillRect(
                ann.data.pointerX - handleSize / 2,
                ann.data.pointerY - handleSize / 2,
                handleSize,
                handleSize,
            );
            overlayCtx.strokeRect(
                ann.data.pointerX - handleSize / 2,
                ann.data.pointerY - handleSize / 2,
                handleSize,
                handleSize,
            );
        }

        overlayCtx.restore();
    }

    function getResizeHandles(
        bounds: { x: number; y: number; width: number; height: number },
        padding: number,
    ) {
        const x = bounds.x - padding;
        const y = bounds.y - padding;
        const w = bounds.width + padding * 2;
        const h = bounds.height + padding * 2;

        return [
            { id: "nw", x: x, y: y },
            { id: "n", x: x + w / 2, y: y },
            { id: "ne", x: x + w, y: y },
            { id: "e", x: x + w, y: y + h / 2 },
            { id: "se", x: x + w, y: y + h },
            { id: "s", x: x + w / 2, y: y + h },
            { id: "sw", x: x, y: y + h },
            { id: "w", x: x, y: y + h / 2 },
        ];
    }

    function hitTestHandle(x: number, y: number): string | null {
        if (!toolStore.selectedAnnotationId) return null;
        const ann = tab.annotations.find(
            (a) => a.id === toolStore.selectedAnnotationId,
        );
        if (!ann) return null;

        const bounds = getAnnotationBounds(ann);
        const padding = 4;
        const handles = getResizeHandles(bounds, padding);
        const hitThreshold = 6 / scale; // Pixel radius for hit

        for (const h of handles) {
            if (
                Math.abs(x - h.x) < hitThreshold &&
                Math.abs(y - h.y) < hitThreshold
            ) {
                return h.id;
            }
        }

        // Check pointer handle
        if (ann.type === "step" && ann.data?.pointerX !== undefined) {
            if (
                Math.abs(x - ann.data.pointerX) < hitThreshold &&
                Math.abs(y - ann.data.pointerY) < hitThreshold
            ) {
                return "pointer";
            }
        }

        return null;
    }

    function renderAnnotation(ann: Annotation) {
        if (!overlayCtx) return;
        overlayCtx.save();
        overlayCtx.strokeStyle = ann.color;
        overlayCtx.lineWidth = ann.strokeWidth;
        overlayCtx.lineCap = "round";
        overlayCtx.lineJoin = "round";

        switch (ann.type) {
            case "pen":
            case "highlighter":
                if (ann.data?.points && ann.data.points.length > 0) {
                    if (ann.type === "highlighter") {
                        overlayCtx.globalAlpha = 0.5;
                    }
                    overlayCtx.beginPath();
                    overlayCtx.moveTo(
                        ann.data.points[0].x,
                        ann.data.points[0].y,
                    );
                    for (let i = 1; i < ann.data.points.length; i++) {
                        overlayCtx.lineTo(
                            ann.data.points[i].x,
                            ann.data.points[i].y,
                        );
                    }
                    overlayCtx.stroke();
                    overlayCtx.globalAlpha = 1.0;
                }
                break;

            case "rectangle":
                if (ann.width !== undefined && ann.height !== undefined) {
                    overlayCtx.strokeRect(ann.x, ann.y, ann.width, ann.height);
                }
                break;

            case "ellipse":
                if (ann.width !== undefined && ann.height !== undefined) {
                    // Ellipse logic handles negative width/height naturally
                    overlayCtx.beginPath();
                    const radiusX = Math.abs(ann.width / 2);
                    const radiusY = Math.abs(ann.height / 2);
                    const centerX = ann.x + ann.width / 2;
                    const centerY = ann.y + ann.height / 2;

                    if (radiusX > 0 && radiusY > 0) {
                        overlayCtx.ellipse(
                            centerX,
                            centerY,
                            radiusX,
                            radiusY,
                            0,
                            0,
                            2 * Math.PI,
                        );
                        overlayCtx.stroke();

                        // Reset line caps for other shapes
                        overlayCtx.lineCap = "round";
                        overlayCtx.lineJoin = "round";
                    }
                }
                break;

            case "step":
                {
                    const radius = 15;
                    const pointerX = ann.data?.pointerX;
                    const pointerY = ann.data?.pointerY;

                    overlayCtx.fillStyle = ann.color;
                    overlayCtx.strokeStyle = ann.color; // For outline if needed, or matched color

                    // Draw pointer tail if it exists and is far enough
                    if (pointerX !== undefined && pointerY !== undefined) {
                        const dx = pointerX - ann.x;
                        const dy = pointerY - ann.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);

                        if (dist > radius) {
                            const angle = Math.atan2(dy, dx);
                            // Calculate tangent points on the circle
                            // Actually, let's just use perpendiculars for a "thick" neck
                            // Or simpler: Triangle from tip to +/- PI/2 at circle
                            const alpha = Math.PI / 2;
                            const p1x =
                                ann.x + radius * Math.cos(angle + alpha);
                            const p1y =
                                ann.y + radius * Math.sin(angle + alpha);
                            const p2x =
                                ann.x + radius * Math.cos(angle - alpha);
                            const p2y =
                                ann.y + radius * Math.sin(angle - alpha);

                            overlayCtx.beginPath();
                            overlayCtx.moveTo(pointerX, pointerY);
                            overlayCtx.lineTo(p1x, p1y);
                            overlayCtx.lineTo(p2x, p2y);
                            overlayCtx.closePath();
                            overlayCtx.fill();
                            overlayCtx.stroke();
                        }
                    }

                    overlayCtx.beginPath();
                    overlayCtx.arc(ann.x, ann.y, radius, 0, 2 * Math.PI);
                    overlayCtx.fill();
                    overlayCtx.stroke(); // Border?
                    // Actually usually step counters don't have borders if filled.
                    // But previous code had stroke().

                    // Draw Number
                    overlayCtx.fillStyle = "#ffffff"; // White text
                    overlayCtx.font = "bold 16px sans-serif";
                    overlayCtx.textAlign = "center";
                    overlayCtx.textBaseline = "middle";
                    overlayCtx.fillText(
                        String(ann.data?.number || "?"),
                        ann.x,
                        ann.y,
                    );
                }
                break;

            case "pixelate":
                if (
                    ann.width !== undefined &&
                    ann.height !== undefined &&
                    Math.abs(ann.width) > 0 &&
                    Math.abs(ann.height) > 0
                ) {
                    // Normalize bounds
                    const x = ann.width < 0 ? ann.x + ann.width : ann.x;
                    const y = ann.height < 0 ? ann.y + ann.height : ann.y;
                    const w = Math.abs(ann.width);
                    const h = Math.abs(ann.height);

                    // 1. Clip to the pixelation rect
                    overlayCtx.beginPath();
                    overlayCtx.rect(x, y, w, h);
                    overlayCtx.save();
                    overlayCtx.clip();

                    // 2. Sample from baseCanvas
                    if (baseCanvas) {
                        try {
                            const pixelationFactor = 0.05;
                            const sw = Math.max(
                                1,
                                Math.floor(w * pixelationFactor),
                            );
                            const sh = Math.max(
                                1,
                                Math.floor(h * pixelationFactor),
                            );

                            const tempCanvas = document.createElement("canvas");
                            tempCanvas.width = sw;
                            tempCanvas.height = sh;
                            const tempCtx = tempCanvas.getContext("2d");

                            if (tempCtx) {
                                tempCtx.imageSmoothingEnabled = false;
                                tempCtx.drawImage(
                                    baseCanvas,
                                    x,
                                    y,
                                    w,
                                    h,
                                    0,
                                    0,
                                    sw,
                                    sh,
                                );

                                overlayCtx.imageSmoothingEnabled = false;
                                overlayCtx.drawImage(
                                    tempCanvas,
                                    0,
                                    0,
                                    sw,
                                    sh,
                                    x,
                                    y,
                                    w,
                                    h,
                                );
                                overlayCtx.imageSmoothingEnabled = true;
                            }
                        } catch (e) {
                            console.error("Pixelation error", e);
                        }
                    }
                    overlayCtx.restore();
                }
                break;

            case "line":
                if (
                    ann.data?.endX !== undefined &&
                    ann.data?.endY !== undefined
                ) {
                    overlayCtx.beginPath();
                    overlayCtx.moveTo(ann.x, ann.y);
                    overlayCtx.lineTo(ann.data.endX, ann.data.endY);
                    overlayCtx.stroke();
                }
                break;

            case "arrow":
                if (
                    ann.data?.endX !== undefined &&
                    ann.data?.endY !== undefined
                ) {
                    const headLength = 15;
                    const dx = ann.data.endX - ann.x;
                    const dy = ann.data.endY - ann.y;
                    const angle = Math.atan2(dy, dx);

                    overlayCtx.beginPath();
                    overlayCtx.moveTo(ann.x, ann.y);
                    overlayCtx.lineTo(ann.data.endX, ann.data.endY);
                    overlayCtx.stroke();

                    // Arrowhead
                    overlayCtx.beginPath();
                    overlayCtx.moveTo(ann.data.endX, ann.data.endY);
                    overlayCtx.lineTo(
                        ann.data.endX -
                            headLength * Math.cos(angle - Math.PI / 6),
                        ann.data.endY -
                            headLength * Math.sin(angle - Math.PI / 6),
                    );
                    overlayCtx.moveTo(ann.data.endX, ann.data.endY);
                    overlayCtx.lineTo(
                        ann.data.endX -
                            headLength * Math.cos(angle + Math.PI / 6),
                        ann.data.endY -
                            headLength * Math.sin(angle + Math.PI / 6),
                    );
                    overlayCtx.stroke();
                }
                break;
        }
        overlayCtx.restore();
    }

    /**
     * Convert mouse event coordinates to canvas coordinates
     */
    function getCanvasCoords(e: MouseEvent): { x: number; y: number } {
        const rect = overlayCanvas.getBoundingClientRect();
        return {
            x: (e.clientX - rect.left) / scale,
            y: (e.clientY - rect.top) / scale,
        };
    }

    /**
     * Find the topmost annotation at the given canvas coordinates
     * (iterate in reverse to pick the one rendered on top)
     */
    function findAnnotationAt(x: number, y: number): Annotation | null {
        for (let i = tab.annotations.length - 1; i >= 0; i--) {
            if (hitTestAnnotation(tab.annotations[i], x, y, scale)) {
                return tab.annotations[i];
            }
        }
        return null;
    }

    // Drag state
    let isDragging = $state(false);
    let potentialDrag = $state(false); // Track if mouse down on object but not yet dragged
    let dragStartX = $state(0);
    let dragStartY = $state(0);

    // ... (rest of imports/state)

    function handleMouseDown(e: MouseEvent) {
        const { x, y } = getCanvasCoords(e);

        // Check for resize handle hit first
        if (
            toolStore.currentTool === "select" &&
            toolStore.selectedAnnotationId
        ) {
            const handle = hitTestHandle(x, y);
            if (handle) {
                isResizing = true;
                resizeHandle = handle;
                const ann = tab.annotations.find(
                    (a) => a.id === toolStore.selectedAnnotationId,
                );
                if (ann) {
                    originalResizeRect = {
                        x: ann.x,
                        y: ann.y,
                        width: ann.width || 0,
                        height: ann.height || 0,
                        pointerX: ann.data?.pointerX,
                        pointerY: ann.data?.pointerY,
                    };
                    dragStartX = x;
                    dragStartY = y;
                }
                return;
            }
        }

        if (toolStore.currentTool === "select") {
            const hit = findAnnotationAt(x, y);

            if (hit) {
                // If clicking a different annotation, select it
                if (toolStore.selectedAnnotationId !== hit.id) {
                    toolStore.selectAnnotation(hit.id);
                }

                potentialDrag = true; // Wait for move to confirm drag
                dragStartX = x;
                dragStartY = y;
                dragAnnotationStartX = hit.x;
                dragAnnotationStartY = hit.y;
            } else {
                // Clicked on empty space - clear selection
                toolStore.clearSelection();
            }
        } else {
            // Start drawing
            isDrawing = true;
            const id = crypto.randomUUID();

            // Initial shape configuration
            drawingAnnotation = {
                id,
                type: toolStore.currentTool as any,
                x,
                y,
                color: toolStore.currentColor,
                strokeWidth: toolStore.currentStrokeWidth,
                width: 0,
                height: 0,
                data: {},
            };

            // Tool specific initialization
            if (
                toolStore.currentTool === "pen" ||
                toolStore.currentTool === "highlighter"
            ) {
                drawingAnnotation.data = { points: [{ x, y }] };
                drawingAnnotation.width = 0;
                drawingAnnotation.height = 0;
            } else if (
                toolStore.currentTool === "line" ||
                toolStore.currentTool === "arrow"
            ) {
                drawingAnnotation.data = { endX: x, endY: y };
            } else if (toolStore.currentTool === "step") {
                // Dynamic step counting: Max + 1
                const existingSteps = tab.annotations
                    .filter(
                        (a) =>
                            a.type === "step" &&
                            typeof a.data?.number === "number",
                    )
                    .map((a) => a.data.number);
                const nextNum =
                    existingSteps.length > 0
                        ? Math.max(...existingSteps) + 1
                        : 1;

                drawingAnnotation.data = { number: nextNum };
                drawingAnnotation.width = 30; // 2 * radius (15)
                drawingAnnotation.height = 30;
            }

            toolStore.clearSelection();
            renderOverlay();
        }
    }

    function handleMouseMove(e: MouseEvent) {
        const { x, y } = getCanvasCoords(e);

        if (
            isResizing &&
            toolStore.selectedAnnotationId &&
            originalResizeRect
        ) {
            handleResizeMove(x, y);
            return;
        }

        if (isDrawing && drawingAnnotation) {
            // Update drawing
            if (
                drawingAnnotation.type === "pen" ||
                drawingAnnotation.type === "highlighter"
            ) {
                drawingAnnotation.data.points.push({ x, y });
                drawingAnnotation.width = Math.max(
                    drawingAnnotation.width || 0,
                    Math.abs(x - drawingAnnotation.x),
                );
                drawingAnnotation.height = Math.max(
                    drawingAnnotation.height || 0,
                    Math.abs(y - drawingAnnotation.y),
                );
            } else if (
                drawingAnnotation.type === "rectangle" ||
                drawingAnnotation.type === "ellipse" ||
                drawingAnnotation.type === "pixelate"
            ) {
                drawingAnnotation.width = x - drawingAnnotation.x;
                drawingAnnotation.height = y - drawingAnnotation.y;
            } else if (
                drawingAnnotation.type === "line" ||
                drawingAnnotation.type === "arrow"
            ) {
                drawingAnnotation.data.endX = x;
                drawingAnnotation.data.endY = y;
                drawingAnnotation.width = Math.abs(x - drawingAnnotation.x);
                drawingAnnotation.height = Math.abs(y - drawingAnnotation.y);
            } else if (drawingAnnotation.type === "step") {
                drawingAnnotation.data.pointerX = x;
                drawingAnnotation.data.pointerY = y;
                // Update bounds to encompass pointer
                const r = 15;
                const top = Math.min(drawingAnnotation.y - r, y);
                const left = Math.min(drawingAnnotation.x - r, x);
                const bottom = Math.max(drawingAnnotation.y + r, y);
                const right = Math.max(drawingAnnotation.x + r, x);
                drawingAnnotation.width = right - left;
                drawingAnnotation.height = bottom - top;
            }

            renderOverlay();
            return;
        }

        // Handle potential drag becoming real drag
        if (potentialDrag && !isDragging) {
            const dx = Math.abs(x - dragStartX);
            const dy = Math.abs(y - dragStartY);
            if (dx > 3 || dy > 3) {
                // Drag threshold
                isDragging = true;
            }
        }

        if (!isDragging || !toolStore.selectedAnnotationId) return;

        // Selection Dragging Logic
        const dx = x - dragStartX;
        const dy = y - dragStartY;

        const ann = tab.annotations.find(
            (a: Annotation) => a.id === toolStore.selectedAnnotationId,
        );
        if (!ann) return;

        // Calculate new position
        const newX = dragAnnotationStartX + dx;
        const newY = dragAnnotationStartY + dy;

        // For paths (pen/highlighter), we need to offset all points
        if (
            (ann.type === "pen" || ann.type === "highlighter") &&
            ann.data?.points
        ) {
            const originalAnn = tabStore.getAnnotation(tab.id, ann.id);
            if (originalAnn?.data?.points) {
                const offsetX = newX - ann.x;
                const offsetY = newY - ann.y;

                const newPoints = ann.data.points.map(
                    (p: { x: number; y: number }) => ({
                        x: p.x + offsetX,
                        y: p.y + offsetY,
                    }),
                );

                tabStore.updateAnnotation(tab.id, ann.id, {
                    x: newX,
                    y: newY,
                    data: { ...ann.data, points: newPoints },
                });
            } else {
                tabStore.updateAnnotation(tab.id, ann.id, { x: newX, y: newY });
            }
        } else if (
            (ann.type === "line" || ann.type === "arrow") &&
            ann.data?.endX !== undefined
        ) {
            // For lines/arrows, also move the end point
            const endXOffset = ann.data.endX - ann.x;
            const endYOffset = ann.data.endY - ann.y;

            tabStore.updateAnnotation(tab.id, ann.id, {
                x: newX,
                y: newY,
                data: {
                    ...ann.data,
                    endX: newX + endXOffset,
                    endY: newY + endYOffset,
                },
            });
        } else if (ann.type === "step" && ann.data?.pointerX !== undefined) {
            const offsetX = newX - ann.x;
            const offsetY = newY - ann.y;
            tabStore.updateAnnotation(tab.id, ann.id, {
                x: newX,
                y: newY,
                data: {
                    ...ann.data,
                    pointerX: ann.data.pointerX + offsetX,
                    pointerY: ann.data.pointerY + offsetY,
                },
            });
        } else {
            // Simple case: just update x, y
            tabStore.updateAnnotation(tab.id, ann.id, { x: newX, y: newY });
        }
    }

    function handleResizeMove(x: number, y: number) {
        if (!resizeHandle || !originalResizeRect) return;

        let { x: ax, y: ay, width: aw, height: ah } = originalResizeRect;
        const dx = x - dragStartX;
        const dy = y - dragStartY;

        // Simple resizing logic for Rect/Ellipse
        // For lines/paths, resizing is harder (scaling).
        // Let's implement rect/ellipse resizing first.

        let newX = ax;
        let newY = ay;
        let newW = aw;
        let newH = ah;

        switch (resizeHandle) {
            case "e":
                newW += dx;
                break;
            case "w":
                newX += dx;
                newW -= dx;
                break;
            case "s":
                newH += dy;
                break;
            case "n":
                newY += dy;
                newH -= dy;
                break;
            case "se":
                newW += dx;
                newH += dy;
                break;
            case "sw":
                newX += dx;
                newW -= dx;
                newH += dy;
                break;
            case "ne":
                newY += dy;
                newW += dx;
                newH -= dy;
                break;
            case "nw":
                newX += dx;
                newY += dy;
                newW -= dx;
                newH -= dy;
                break;
        }

        tabStore.updateAnnotation(tab.id, toolStore.selectedAnnotationId!, {
            x: newX,
            y: newY,
            width: newW,
            height: newH,
        });
    }

    function handleMouseUp() {
        if (isDrawing && drawingAnnotation) {
            // Finalize drawing
            // addAnnotation already pushes history
            tabStore.addAnnotation(tab.id, $state.snapshot(drawingAnnotation));
            drawingAnnotation = null;
            renderOverlay();
        } else if (isDragging || isResizing) {
            // Commit history ONLY if we actually dragged or resized
            tabStore.commitHistory(tab.id);
        }
        isDrawing = false;
        isDragging = false;
        potentialDrag = false;
        isResizing = false;
        resizeHandle = null;
        originalResizeRect = null;
    }

    function handleMouseLeave() {
        // If mouse leaves canvas while drawing/dragging, treat as mouseup
        if (isDrawing || isDragging) {
            handleMouseUp();
        }
    }

    function handleKeyDown(e: KeyboardEvent) {
        // Global shortcuts (Undo/Redo)
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "z") {
            e.preventDefault();
            tabStore.undo(tab.id);
            return;
        }
        if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "y") {
            e.preventDefault();
            tabStore.redo(tab.id);
            return;
        }

        // Only handle delete if we have a selection
        if (!toolStore.selectedAnnotationId) return;

        if (e.key === "Delete" || e.key === "Backspace") {
            e.preventDefault();
            deleteSelectedAnnotation();
        }
    }

    function deleteSelectedAnnotation() {
        if (toolStore.selectedAnnotationId) {
            // removeAnnotation already pushes history
            tabStore.removeAnnotation(tab.id, toolStore.selectedAnnotationId);
            toolStore.clearSelection();
        }
    }

    onMount(() => {
        baseCtx = baseCanvas.getContext("2d");
        overlayCtx = overlayCanvas.getContext("2d");

        const observer = new ResizeObserver(() => resizeCanvas());
        observer.observe(canvasContainer);

        // Add global mouseup/mouseleave listeners
        window.addEventListener("mouseup", handleMouseUp);
        window.addEventListener("mouseleave", handleMouseLeave);
        // Add global keydown for shortcuts (so it works even if canvas isn't focused)
        window.addEventListener("keydown", handleKeyDown);

        return () => {
            observer.disconnect();
            window.removeEventListener("mouseup", handleMouseUp);
            window.removeEventListener("mouseleave", handleMouseLeave);
            window.removeEventListener("keydown", handleKeyDown);
        };
    });
</script>

<div class="canvas-container" bind:this={canvasContainer}>
    <div class="canvas-stack">
        <canvas bind:this={baseCanvas} class="layer base"></canvas>
        <canvas
            bind:this={overlayCanvas}
            class="layer overlay"
            onmousedown={handleMouseDown}
            onmousemove={handleMouseMove}
            tabindex="0"
        ></canvas>
    </div>
</div>

<style>
    .canvas-container {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: #333; /* Dark background for canvas area */
        overflow: auto;
    }

    .canvas-stack {
        position: relative;
        box-shadow: 0 0 20px rgba(0, 0, 0, 0.5);
        background-color: #fff;
        background-image: linear-gradient(45deg, #ccc 25%, transparent 25%),
            linear-gradient(-45deg, #ccc 25%, transparent 25%),
            linear-gradient(45deg, transparent 75%, #ccc 75%),
            linear-gradient(-45deg, transparent 75%, #ccc 75%);
        background-size: 20px 20px;
        background-position:
            0 0,
            0 10px,
            10px -10px,
            -10px 0px;
    }

    .layer {
        display: block;
        image-rendering: pixelated; /* Sharp scaling */
    }

    .overlay {
        position: absolute;
        top: 0;
        left: 0;
        pointer-events: auto; /* Allow interactions */
        outline: none; /* Remove focus outline */
    }
</style>
