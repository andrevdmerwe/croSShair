<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import type { TabData, Annotation } from "$lib/types";

    import { invoke } from "@tauri-apps/api/core";
    import { toastStore } from "$lib/stores/toast.svelte";

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

    // Re-render when annotations change
    $effect(() => {
        if (imageLoaded && overlayCtx) {
            // We depend on tab.annotations to trigger re-renders
            // Just referencing it is enough for Svelte 5 dependency tracking
            const _ = tab.annotations;
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

        tab.annotations.forEach(renderAnnotation);
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
                if (ann.width && ann.height) {
                    overlayCtx.strokeRect(ann.x, ann.y, ann.width, ann.height);
                }
                break;

            case "ellipse":
                if (ann.width && ann.height) {
                    overlayCtx.beginPath();
                    overlayCtx.ellipse(
                        ann.x + ann.width / 2,
                        ann.y + ann.height / 2,
                        Math.abs(ann.width / 2),
                        Math.abs(ann.height / 2),
                        0,
                        0,
                        2 * Math.PI,
                    );
                    overlayCtx.stroke();
                }
                break;

            // Other types implementation...
        }
        overlayCtx.restore();
    }

    onMount(() => {
        baseCtx = baseCanvas.getContext("2d");
        overlayCtx = overlayCanvas.getContext("2d");

        const observer = new ResizeObserver(() => resizeCanvas());
        observer.observe(canvasContainer);

        return () => observer.disconnect();
    });
</script>

<div class="canvas-container" bind:this={canvasContainer}>
    <div class="canvas-stack">
        <canvas bind:this={baseCanvas} class="layer base"></canvas>
        <canvas bind:this={overlayCanvas} class="layer overlay"></canvas>
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
    }
</style>
