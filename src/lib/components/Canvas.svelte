<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import type { TabData, Annotation } from "$lib/types";

    import { invoke } from "@tauri-apps/api/core";
    import { toastStore } from "$lib/stores/toast.svelte";
    import { toolStore } from "$lib/stores/tools.svelte";
    import { tabStore } from "$lib/stores/tabs.svelte";
    import { getAnnotationBounds, hitTestAnnotation } from "$lib/utils/hitTest";
    import { themeStore } from "$lib/stores/theme.svelte";

    let { tab } = $props<{ tab: TabData }>();

    let canvasContainer: HTMLDivElement;
    let baseCanvas: HTMLCanvasElement;
    let overlayCanvas: HTMLCanvasElement;
    let baseCtx: CanvasRenderingContext2D | null = null;
    let overlayCtx: CanvasRenderingContext2D | null = null;

    // Theme integration for Canvas rendering
    let accentColor = $state("#0088ff");
    let handleFillColor = $state("#ffffff");

    $effect(() => {
        // React to theme changes
        const _ = themeStore.currentTheme;
        const __ = themeStore.customOverrides; // Deep dependency handled by svelte 5 state?
        // Note: customOverrides is a rune, accessing it triggers dependency.

        if (typeof window !== "undefined") {
            setTimeout(() => {
                // Small delay to allow CSS to apply?
                const style = getComputedStyle(document.documentElement);
                accentColor =
                    style.getPropertyValue("--accent").trim() || "#0088ff";
                // Handles usually white or bright, could be text-main
                handleFillColor =
                    style.getPropertyValue("--bg-primary").trim() || "#ffffff";
                renderOverlay();
            }, 10);
        }
    });

    export async function exportImage(): Promise<string> {
        if (!imageLoaded) throw new Error("Image not loaded");

        // Create offscreen canvas at full resolution
        const offCanvas = document.createElement("canvas");
        offCanvas.width = image.width;
        offCanvas.height = image.height;
        const ctx = offCanvas.getContext("2d");

        if (!ctx) throw new Error("Failed to create context");

        // 1. Draw Base Image
        ctx.drawImage(image, 0, 0);

        // 2. Draw Annotations (re-use renderAnnotation logic but on this context)
        tab.annotations.forEach((ann) => renderAnnotationOnContext(ctx, ann));

        return offCanvas.toDataURL("image/png");
    }

    // Refactor renderAnnotation to helper that takes context
    function renderAnnotationOnContext(
        ctx: CanvasRenderingContext2D,
        ann: Annotation,
    ) {
        ctx.save();
        ctx.strokeStyle = ann.color;
        ctx.lineWidth = ann.strokeWidth;
        ctx.lineCap = "round";
        ctx.lineJoin = "round";

        switch (ann.type) {
            case "pen":
            case "highlighter":
                if (ann.data?.points && ann.data.points.length > 0) {
                    if (ann.type === "highlighter") {
                        ctx.globalAlpha = 0.5;
                    }
                    ctx.beginPath();
                    ctx.moveTo(ann.data.points[0].x, ann.data.points[0].y);
                    for (let i = 1; i < ann.data.points.length; i++) {
                        ctx.lineTo(ann.data.points[i].x, ann.data.points[i].y);
                    }
                    ctx.stroke();
                    ctx.globalAlpha = 1.0;
                }
                break;

            case "rectangle":
                if (ann.width !== undefined && ann.height !== undefined) {
                    ctx.strokeRect(ann.x, ann.y, ann.width, ann.height);
                }
                break;

            case "ellipse":
                if (ann.width !== undefined && ann.height !== undefined) {
                    ctx.beginPath();
                    const radiusX = Math.abs(ann.width / 2);
                    const radiusY = Math.abs(ann.height / 2);
                    const centerX = ann.x + ann.width / 2;
                    const centerY = ann.y + ann.height / 2;

                    if (radiusX > 0 && radiusY > 0) {
                        ctx.ellipse(
                            centerX,
                            centerY,
                            radiusX,
                            radiusY,
                            0,
                            0,
                            2 * Math.PI,
                        );
                        ctx.stroke();
                    }
                }
                break;

            case "step":
                {
                    const radius = 15;
                    const pointerX = ann.data?.pointerX;
                    const pointerY = ann.data?.pointerY;

                    ctx.fillStyle = ann.color;
                    ctx.strokeStyle = ann.color;

                    if (pointerX !== undefined && pointerY !== undefined) {
                        const dx = pointerX - ann.x;
                        const dy = pointerY - ann.y;
                        const dist = Math.sqrt(dx * dx + dy * dy);

                        if (dist > radius) {
                            const angle = Math.atan2(dy, dx);
                            const alpha = Math.PI / 2;
                            const p1x =
                                ann.x + radius * Math.cos(angle + alpha);
                            const p1y =
                                ann.y + radius * Math.sin(angle + alpha);
                            const p2x =
                                ann.x + radius * Math.cos(angle - alpha);
                            const p2y =
                                ann.y + radius * Math.sin(angle - alpha);

                            ctx.beginPath();
                            ctx.moveTo(pointerX, pointerY);
                            ctx.lineTo(p1x, p1y);
                            ctx.lineTo(p2x, p2y);
                            ctx.closePath();
                            ctx.fill();
                            ctx.stroke();
                        }
                    }

                    ctx.beginPath();
                    ctx.arc(ann.x, ann.y, radius, 0, 2 * Math.PI);
                    ctx.fill();
                    ctx.stroke();

                    ctx.fillStyle = "#ffffff";
                    ctx.font = "bold 16px sans-serif";
                    ctx.textAlign = "center";
                    ctx.textBaseline = "middle";
                    ctx.fillText(String(ann.data?.number || "?"), ann.x, ann.y);
                }
                break;

            case "pixelate":
                // Pixelate is tricky because it relies on sampling the base image.
                // Since 'ctx' here is a fresh canvas with base image already drawn,
                // we can just use the same pixelation logic but targeting itself?
                // Actually, the original implementation samples 'baseCanvas'.
                // We should pass baseCanvas or just rely on 'image' being the source.
                if (
                    ann.width !== undefined &&
                    ann.height !== undefined &&
                    Math.abs(ann.width) > 0 &&
                    Math.abs(ann.height) > 0
                ) {
                    const x = ann.width < 0 ? ann.x + ann.width : ann.x;
                    const y = ann.height < 0 ? ann.y + ann.height : ann.y;
                    const w = Math.abs(ann.width);
                    const h = Math.abs(ann.height);

                    ctx.beginPath();
                    ctx.rect(x, y, w, h);
                    ctx.save();
                    ctx.clip();

                    // Pixelation logic
                    const blockSize = Math.max(1, ann.strokeWidth || 10);
                    const sw = Math.floor(w / blockSize);
                    const sh = Math.floor(h / blockSize);
                    const safeSw = Math.max(1, sw);
                    const safeSh = Math.max(1, sh);

                    // Draw image scaled down then up
                    ctx.imageSmoothingEnabled = false;
                    // We use 'image' as source which is available in scope
                    // The trick is draw small to offscreen then draw back large.
                    // Since we don't want to create another temp canvas if possible...
                    // Let's create temp canvas like original code
                    const tempCanvas = document.createElement("canvas");
                    tempCanvas.width = safeSw;
                    tempCanvas.height = safeSh;
                    const tempCtx = tempCanvas.getContext("2d");
                    if (tempCtx) {
                        tempCtx.imageSmoothingEnabled = false;
                        tempCtx.drawImage(
                            image,
                            x,
                            y,
                            w,
                            h,
                            0,
                            0,
                            safeSw,
                            safeSh,
                        );
                        ctx.drawImage(
                            tempCanvas,
                            0,
                            0,
                            safeSw,
                            safeSh,
                            x,
                            y,
                            w,
                            h,
                        );
                    }
                    ctx.imageSmoothingEnabled = true;
                    ctx.restore();
                }
                break;

            case "line":
                if (
                    ann.data?.endX !== undefined &&
                    ann.data?.endY !== undefined
                ) {
                    ctx.beginPath();
                    ctx.moveTo(ann.x, ann.y);
                    ctx.lineTo(ann.data.endX, ann.data.endY);
                    ctx.stroke();
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

                    ctx.beginPath();
                    ctx.moveTo(ann.x, ann.y);
                    ctx.lineTo(ann.data.endX, ann.data.endY);
                    ctx.stroke();

                    ctx.beginPath();
                    ctx.moveTo(ann.data.endX, ann.data.endY);
                    ctx.lineTo(
                        ann.data.endX -
                            headLength * Math.cos(angle - Math.PI / 6),
                        ann.data.endY -
                            headLength * Math.sin(angle - Math.PI / 6),
                    );
                    ctx.moveTo(ann.data.endX, ann.data.endY);
                    ctx.lineTo(
                        ann.data.endX -
                            headLength * Math.cos(angle + Math.PI / 6),
                        ann.data.endY -
                            headLength * Math.sin(angle + Math.PI / 6),
                    );
                    ctx.stroke();
                }
                break;

            case "text":
                if (ann.data?.text) {
                    ctx.fillStyle = ann.color;
                    const fontSize = 14 + ann.strokeWidth * 4;
                    const lineHeight = fontSize * 1.2;
                    ctx.font = `${fontSize}px sans-serif`;
                    ctx.textBaseline = "top";

                    const lines = ann.data.text.split("\n");
                    lines.forEach((line: string, i: number) => {
                        ctx.fillText(line, ann.x, ann.y + i * lineHeight);
                    });
                }
                break;
        }
        ctx.restore();
    }

    // This function is called by renderOverlay to draw annotations on the overlay canvas
    function renderAnnotation(ann: Annotation) {
        if (overlayCtx) {
            renderAnnotationOnContext(overlayCtx, ann);
        }
    }

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
        // Raw coordinates before normalization (for lines/arrows)
        rawX?: number;
        rawY?: number;
        pointerX?: number;
        pointerY?: number;
        endX?: number;
        endY?: number;
    } | null>(null);

    // Sync slider <-> selected annotation

    // Update toolstore when selection changes
    $effect(() => {
        // This effect runs when selectedAnnotationId changes
        if (toolStore.selectedAnnotationId) {
            const ann = tab.annotations.find(
                (a) => a.id === toolStore.selectedAnnotationId,
            );
            if (ann) {
                // Sync Stroke Width
                // Verify tool type matches to avoid setting "pen" width to "pixelate" density if switching tools (but we clear selection on tool switch anyway)
                if (
                    ann.strokeWidth &&
                    toolStore.currentStrokeWidth !== ann.strokeWidth
                ) {
                    // Use untracked to avoid loop? Svelte 5 handling...
                    // checking value to avoid infinite loop
                    toolStore.setStrokeWidth(ann.strokeWidth);
                }
                // Sync Color
                if (ann.color && toolStore.currentColor !== ann.color) {
                    toolStore.setColor(ann.color);
                }
            }
        }
    });
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
        overlayCtx.strokeStyle = accentColor;
        overlayCtx.lineWidth = 2; // Fixed width for selection
        overlayCtx.setLineDash([6, 4]);
        overlayCtx.strokeRect(
            bounds.x - padding,
            bounds.y - padding,
            bounds.width + padding * 2,
            bounds.height + padding * 2,
        );

        // Draw resize handles
        const resizableTypes = ["rectangle", "ellipse", "crop", "pixelate"];
        const isResizable = resizableTypes.includes(ann.type);

        if (isResizable) {
            overlayCtx.setLineDash([]);
            overlayCtx.fillStyle = handleFillColor;
            overlayCtx.strokeStyle = accentColor;
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
        }

        // Line/Arrow Handles (Start/End)
        if (ann.type === "line" || ann.type === "arrow") {
            if (ann.data?.endX !== undefined && ann.data?.endY !== undefined) {
                overlayCtx.setLineDash([]);
                overlayCtx.fillStyle = handleFillColor;
                overlayCtx.strokeStyle = accentColor;
                const handleSize = 8 / scale;

                const points = [
                    { x: ann.x, y: ann.y }, // Start
                    { x: ann.data.endX, y: ann.data.endY }, // End
                ];

                points.forEach((p) => {
                    overlayCtx!.fillRect(
                        p.x - handleSize / 2,
                        p.y - handleSize / 2,
                        handleSize,
                        handleSize,
                    );
                    overlayCtx!.strokeRect(
                        p.x - handleSize / 2,
                        p.y - handleSize / 2,
                        handleSize,
                        handleSize,
                    );
                });
            }
        }

        // Special handle for Step Pointer
        if (ann.type === "step" && ann.data?.pointerX !== undefined) {
            const handleSize = 8 / scale; // Define here as fallback
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

        const resizableTypes = ["rectangle", "ellipse", "crop", "pixelate"];
        if (resizableTypes.includes(ann.type)) {
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
        }

        // Check Line/Arrow handles
        if (ann.type === "line" || ann.type === "arrow") {
            const hitThreshold = 6 / scale;
            if (
                Math.abs(x - ann.x) < hitThreshold &&
                Math.abs(y - ann.y) < hitThreshold
            ) {
                return "start";
            }
            if (ann.data?.endX !== undefined && ann.data?.endY !== undefined) {
                if (
                    Math.abs(x - ann.data.endX) < hitThreshold &&
                    Math.abs(y - ann.data.endY) < hitThreshold
                ) {
                    return "end";
                }
            }
        }

        // Check pointer handle
        if (ann.type === "step" && ann.data?.pointerX !== undefined) {
            const hitThreshold = 6 / scale;
            if (
                Math.abs(x - ann.data.pointerX) < hitThreshold &&
                Math.abs(y - ann.data.pointerY) < hitThreshold
            ) {
                return "pointer";
            }
        }

        return null;
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

        // 1. Check for Resize Handle Hit (on selected object)
        if (toolStore.selectedAnnotationId) {
            const handle = hitTestHandle(x, y);
            if (handle) {
                isResizing = true;
                resizeHandle = handle;
                const ann = tab.annotations.find(
                    (a) => a.id === toolStore.selectedAnnotationId,
                );
                if (ann) {
                    // Use getAnnotationBounds to ensure we start effectively normalized
                    // But we need to be careful: simple X/Y/W/H is what we want for rectangles.
                    // If we use getAnnotationBounds() we lose the fact that width might be negative in the store.
                    // BUT for the opposite-side resizing bug, normalizing HERE is exactly what we want.
                    // We treat the current VISUAL state as the truth for resizing logic.
                    const bounds = getAnnotationBounds(ann);

                    originalResizeRect = {
                        x: bounds.x,
                        y: bounds.y,
                        width: bounds.width,
                        height: bounds.height,
                        // Store raw coordinates for line resizing which needs exact start/end points
                        rawX: ann.x,
                        rawY: ann.y,
                        pointerX: ann.data?.pointerX,
                        pointerY: ann.data?.pointerY,
                        endX: ann.data?.endX,
                        endY: ann.data?.endY,
                    };
                    dragStartX = x;
                    dragStartY = y;
                }
                return;
            }
        }

        // 2. Check for Object Hit (Global Selection)
        // We prefer selecting an object over drawing, unless we are maybe in a 'forcing' mode (not spec'd).
        const hit = findAnnotationAt(x, y);
        if (hit) {
            // Clicked on an object
            if (toolStore.selectedAnnotationId !== hit.id) {
                // PRE-SYNC: Update tool properties to match object BEFORE selection
                // This prevents the "Update Annotation" effect from seeing a mismatch
                // and overwriting the object's style with the previous tool style.
                if (hit.color) toolStore.setColor(hit.color);
                if (hit.strokeWidth) toolStore.setStrokeWidth(hit.strokeWidth);

                toolStore.selectAnnotation(hit.id);
            }

            // Initiate Drag
            potentialDrag = true;
            dragStartX = x;
            dragStartY = y;
            dragAnnotationStartX = hit.x;
            dragAnnotationStartY = hit.y;
            return;
        }

        // 3. No Hit - Clicked Empty Space
        // Deselect if we had a selection
        if (toolStore.selectedAnnotationId) {
            toolStore.clearSelection();
            renderOverlay();
        }

        // 4. Start Drawing (if tool is not 'select')
        if (toolStore.currentTool !== "select") {
            // Text Tool Handling
            if (toolStore.currentTool === "text") {
                startTextEditing(x, y);
                return;
            }

            isDrawing = true;
            dragStartX = x;
            dragStartY = y;

            const id = crypto.randomUUID();
            drawingAnnotation = {
                id,
                type: toolStore.currentTool as any,
                x,
                y,
                width: 0,
                height: 0,
                color: toolStore.currentColor, // Use store color
                strokeWidth: toolStore.currentStrokeWidth, // Use store width
                data: {},
            };

            // Initialize specific tool data
            if (
                toolStore.currentTool === "pen" ||
                toolStore.currentTool === "highlighter"
            ) {
                drawingAnnotation.data = { points: [{ x, y }] };
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

                drawingAnnotation.data = {
                    number: nextNum,
                    pointerX: x, // Default pointer straight down or center?
                    pointerY: y + 25, //
                };
                // Initial size for circle
                drawingAnnotation.width = 30;
                drawingAnnotation.height = 30;
            }
        }
    }

    // Text Editing State
    let isEditingText = $state(false);
    let editingTextId = $state<string | null>(null);
    let textInput: HTMLTextAreaElement | null = null;
    let textInputX = $state(0);
    let textInputY = $state(0);
    let textInputWidth = $state(0);
    let textInputHeight = $state(0);
    let textInputValue = $state("");
    let textInputColor = $state("#ff0000");
    let textInputFontSize = $state(24);

    function startTextEditing(
        x: number,
        y: number,
        existingId: string | null = null,
    ) {
        if (isEditingText) commitTextEditing();

        isEditingText = true;
        editingTextId = existingId;

        if (existingId) {
            const ann = tab.annotations.find((a) => a.id === existingId);
            if (ann) {
                textInputValue = ann.data?.text || "";
                textInputX = ann.x;
                textInputY = ann.y;
                textInputWidth = ann.width ?? 0;
                textInputHeight = ann.height ?? 0;
                textInputColor = ann.color;
                textInputFontSize = 14 + ann.strokeWidth * 4;
            }
        } else {
            // New text
            textInputValue = "";
            textInputX = x;
            textInputY = y;
            textInputWidth = 0; // Auto-size will handle it or min-width
            textInputHeight = 0;
            textInputColor = toolStore.currentColor;
            textInputFontSize = 14 + toolStore.currentStrokeWidth * 4;
        }

        // Focus next tick
        setTimeout(() => textInput?.focus(), 10);

        // Remove global listener
        window.removeEventListener("keydown", handleKeyDown);
    }

    function commitTextEditing() {
        if (!isEditingText) return;

        // Restore global listener
        window.addEventListener("keydown", handleKeyDown);

        if (textInputValue.trim()) {
            // Calculate dimensions based on content
            const fontSize = textInputFontSize;
            const lineHeight = fontSize * 1.2;
            const lines = textInputValue.split("\n");
            const lineCount = lines.length;

            let maxWidth = 0;
            if (overlayCtx) {
                overlayCtx.save();
                overlayCtx.font = `${fontSize}px sans-serif`;
                lines.forEach((line) => {
                    const metrics = overlayCtx!.measureText(line);
                    maxWidth = Math.max(maxWidth, metrics.width);
                });
                overlayCtx.restore();
            } else {
                maxWidth =
                    Math.max(...lines.map((l) => l.length)) * (fontSize * 0.6);
            }

            const width = Math.ceil(maxWidth);
            const height = Math.ceil(lineCount * lineHeight);

            if (editingTextId) {
                // Update existing
                tabStore.updateAnnotation(tab.id, editingTextId, {
                    width,
                    height,
                    data: { text: textInputValue },
                });
            } else {
                // Create new
                tabStore.addAnnotation(tab.id, {
                    id: crypto.randomUUID(),
                    type: "text",
                    x: textInputX,
                    y: textInputY,
                    width,
                    height,
                    color: textInputColor,
                    strokeWidth: toolStore.currentStrokeWidth,
                    data: { text: textInputValue },
                });
            }
        } else if (editingTextId) {
            tabStore.removeAnnotation(tab.id, editingTextId);
        }

        isEditingText = false;
        editingTextId = null;
        renderOverlay();
    }

    function handleDoubleClick(e: MouseEvent) {
        const { x, y } = getCanvasCoords(e);
        const hit = findAnnotationAt(x, y);
        if (hit && hit.type === "text") {
            startTextEditing(x, y, hit.id);
        }
    }

    function handleMouseMove(e: MouseEvent) {
        const { x, y } = getCanvasCoords(e);

        // Update cursor based on hover
        if (!isDrawing && !isDragging && !isResizing) {
            const handle = hitTestHandle(x, y);
            if (handle) {
                const cursorMap: Record<string, string> = {
                    n: "ns-resize",
                    s: "ns-resize",
                    e: "ew-resize",
                    w: "ew-resize",
                    nw: "nwse-resize",
                    se: "nwse-resize",
                    ne: "nesw-resize",
                    sw: "nesw-resize",
                    pointer: "grab",
                };
                overlayCanvas.style.cursor = cursorMap[handle] || "default";
            } else if (findAnnotationAt(x, y)) {
                overlayCanvas.style.cursor = "move";
            } else {
                overlayCanvas.style.cursor =
                    toolStore.currentTool === "select"
                        ? "default"
                        : "crosshair";
            }
        }

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
            // Decoupled movement: Dragging the head (x,y) leaves the pointer tip pinned.
            // We simply update x/y without applying offset to pointerX/pointerY.
            tabStore.updateAnnotation(tab.id, ann.id, {
                x: newX,
                y: newY,
                // Explicitly do NOT update pointerX/Y
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

            case "start":
                if (
                    originalResizeRect.rawX !== undefined &&
                    originalResizeRect.rawY !== undefined
                ) {
                    tabStore.updateAnnotation(
                        tab.id,
                        toolStore.selectedAnnotationId!,
                        {
                            x: originalResizeRect.rawX + dx,
                            y: originalResizeRect.rawY + dy,
                        },
                    );
                }
                return;

            case "end":
                if (
                    originalResizeRect.endX !== undefined &&
                    originalResizeRect.endY !== undefined
                ) {
                    const ann = tab.annotations.find(
                        (a) => a.id === toolStore.selectedAnnotationId,
                    );
                    if (ann) {
                        tabStore.updateAnnotation(tab.id, ann.id, {
                            data: {
                                ...ann.data,
                                endX: originalResizeRect.endX + dx,
                                endY: originalResizeRect.endY + dy,
                            },
                        });
                    }
                }
                return;

            case "pointer":
                // Special case: update pointer X/Y relative to original position
                if (
                    originalResizeRect.pointerX !== undefined &&
                    originalResizeRect.pointerY !== undefined
                ) {
                    const ann = tab.annotations.find(
                        (a) => a.id === toolStore.selectedAnnotationId,
                    );
                    if (ann) {
                        tabStore.updateAnnotation(tab.id, ann.id, {
                            data: {
                                ...ann.data,
                                pointerX: originalResizeRect.pointerX + dx,
                                pointerY: originalResizeRect.pointerY + dy,
                            },
                        });
                    }
                    return; // Skip main bounds update
                }
                break;
        }

        if (resizeHandle === "start" || resizeHandle === "end") {
            // For lines/arrows, we didn't store raw coordinates properly in valid originalResizeRect because of getAnnotationBounds.
            // But actually, we only need to know "original x" vs "normalized x".
            // Let's grab the annotation again to be sure? No, bad pattern.

            // Simplest Fix: For "start"/"end", we just want to move that point by dx/dy relative to when we started dragging.
            // We know `dragStartX/Y`. We know `current x/y`.
            // `delta = x - dragStartX`.
            // We need `originalAnnX` and `originalAnnY`.
            // I'll grab them from `originalResizeRect` assuming I saved them?
            // I saved `endX/endY`! specific properties.
            // Resizing lines:
            // Start Point: ann.x + dx
            // End Point: ann.data.endX + dx

            // BUT `originalResizeRect.x` is NORMALIZED.
            // So I can't trust it is `ann.x`.

            // I will make `handleMouseDown` smarter in previous chunk.
            // I'll update that chunk to store `rawX`, `rawY`.
            return;
        }

        // For Rects/Ellipses/Crop:
        // Normalize negative W/H before saving
        if (newW < 0) {
            newX += newW;
            newW = Math.abs(newW);
        }
        if (newH < 0) {
            newY += newH;
            newH = Math.abs(newH);
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
        // Ignore if default prevented or editing text
        if (e.defaultPrevented) return;

        const target = e.target as HTMLElement;
        if (
            target.tagName === "INPUT" ||
            target.tagName === "TEXTAREA" ||
            target.isContentEditable ||
            target.matches?.("input, textarea")
        )
            return;

        // Also check local state as backup
        if (isEditingText) return;

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
            ondblclick={handleDoubleClick}
            tabindex="0"
        ></canvas>
        {#if isEditingText}
            <textarea
                bind:this={textInput}
                bind:value={textInputValue}
                onblur={commitTextEditing}
                onkeydown={(e) => {
                    e.stopPropagation(); // Stop global shortcuts (Undo, Delete, etc.)
                    if (e.key === "Enter" && !e.shiftKey) {
                        e.preventDefault();
                        commitTextEditing();
                    }
                    if (e.key === "Escape") {
                        isEditingText = false;
                        editingTextId = null;
                        renderOverlay();
                    }
                }}
                style="
                    position: absolute;
                    left: {textInputX * scale}px;
                    top: {textInputY * scale}px;
                    font-size: {textInputFontSize * scale}px;
                    line-height: 1.2;
                    color: {textInputColor};
                    font-family: sans-serif;
                    background: var(--bg-glass);
                    border: 1px dashed var(--accent);
                    padding: 0;
                    margin: 0;
                    outline: none;
                    min-width: 100px;
                    min-height: 1.2em;
                    width: {textInputWidth
                    ? textInputWidth * scale + 'px'
                    : 'auto'};
                    height: {textInputHeight
                    ? textInputHeight * scale + 'px'
                    : 'auto'};
                    z-index: 100;
                    resize: none;
                    overflow: hidden;
                "
            ></textarea>
        {/if}
    </div>
</div>

<style>
    .canvas-container {
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        background: var(--bg-secondary); /* Dark background for canvas area */
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
