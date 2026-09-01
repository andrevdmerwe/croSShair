<script lang="ts">
    import { toolStore } from "$lib/stores/tools.svelte";
    import { tabStore } from "$lib/stores/tabs.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import SettingsModal from "$lib/components/SettingsModal.svelte";
    import type { Tool } from "$lib/types";
    import { onMount } from "svelte";

    const tools: { id: Tool; label: string; icon: string }[] = [
        { id: "pen", label: "Pen", icon: "✎" },
        { id: "highlighter", label: "Highlighter", icon: "🖊" },
        { id: "highlight", label: "Highlight", icon: "■" },
        { id: "text", label: "Text", icon: "T" },
        { id: "speechBubble", label: "Speech Bubble", icon: "💬" },
        { id: "rectangle", label: "Rectangle", icon: "☐" },
        { id: "ellipse", label: "Ellipse", icon: "◯" },
        { id: "line", label: "Line", icon: "／" },
        { id: "arrow", label: "Arrow", icon: "↗" },
        { id: "step", label: "Step Number", icon: "➊" },
        { id: "pixelate", label: "Pixelate", icon: "░" },
        { id: "crop", label: "Crop", icon: "✂" },
        { id: "pan", label: "Pan (hold Space or middle-drag)", icon: "✋" },
    ];

    // Dynamic palette from settings
    let colors = $derived(settingsStore.settings.toolbarPalette);
    let showSettings = $state(false);

    // Determine if we're in speech bubble context
    let isSpeechBubble = $derived(
        toolStore.currentTool === "speechBubble" ||
        (toolStore.selectedAnnotationId !== null &&
            tabStore.activeTabId !== null &&
            tabStore.getTab(tabStore.activeTabId)?.annotations.find(
                a => a.id === toolStore.selectedAnnotationId
            )?.type === "speechBubble")
    );

    let { onCapture, onOpenImage, onSave, onSaveAs, onCopy } = $props<{
        onCapture?: () => void;
        onOpenImage?: () => void;
        onSave?: () => void;
        onSaveAs?: () => void;
        onCopy?: () => void;
    }>();

    onMount(() => {
        toolStore.setTool(settingsStore.settings.defaultTool as Tool);
        toolStore.setStrokeWidth(settingsStore.settings.defaultStrokeWidth);
        toolStore.setColor(settingsStore.settings.defaultColor);
    });

    function commitChange() {
        if (tabStore.activeTabId && toolStore.selectedAnnotationId) {
            tabStore.commitHistory(tabStore.activeTabId);
        }
    }

    function updateSpeechBubbleData(field: string, value: number) {
        if (toolStore.selectedAnnotationId && tabStore.activeTabId) {
            const ann = tabStore.getTab(tabStore.activeTabId)?.annotations.find(
                a => a.id === toolStore.selectedAnnotationId
            );
            if (ann && ann.type === "speechBubble") {
                tabStore.updateAnnotation(
                    tabStore.activeTabId,
                    toolStore.selectedAnnotationId,
                    { data: { ...ann.data, [field]: value } },
                );
            }
        }
    }
</script>

<div class="palette">
    <!-- Utility Buttons (Top) -->
    <div class="section utility-section">
        <button
            class="tool-btn"
            onclick={() => (showSettings = true)}
            title="Settings"
        >
            <span class="icon">⚙</span>
        </button>
        <button
            class="tool-btn"
            onclick={() => onCapture?.()}
            title="New Screenshot"
        >
            <span class="icon">📷</span>
        </button>
        <button
            class="tool-btn"
            onclick={() => onOpenImage?.()}
            title="Open Image..."
        >
            <span class="icon">📂</span>
        </button>
        <button
            class="tool-btn"
            onclick={() => onCopy?.()}
            title="Copy to Clipboard"
        >
            <span class="icon">📋</span>
        </button>
        <div class="utility-pair">
            <button
                class="tool-btn"
                onclick={() => onSave?.()}
                title="Save (Overwrite)"
            >
                <span class="icon">💾</span>
            </button>
            <button
                class="tool-btn"
                onclick={() => onSaveAs?.()}
                title="Save As..."
            >
                <span class="icon save-as">💾<span class="plus">+</span></span>
            </button>
        </div>
        <div class="utility-pair">
            <button
                class="tool-btn"
                onclick={() =>
                    tabStore.activeTabId && tabStore.undo(tabStore.activeTabId)}
                title="Undo (Ctrl+Z)"
            >
                <span class="icon">↩</span>
            </button>
            <button
                class="tool-btn"
                onclick={() =>
                    tabStore.activeTabId && tabStore.redo(tabStore.activeTabId)}
                title="Redo (Ctrl+Y)"
            >
                <span class="icon">↪</span>
            </button>
        </div>
    </div>

    <div class="divider"></div>

    <!-- Tools grid: 2 columns -->
    <div class="section tools-grid">
        {#each tools as tool}
            <button
                class="tool-btn"
                class:active={toolStore.currentTool === tool.id}
                onclick={() => {
                    toolStore.setTool(tool.id);
                    if (tool.id === "pixelate") {
                        toolStore.setStrokeWidth(
                            settingsStore.settings.defaultPixelDensity,
                        );
                    }
                }}
                title={tool.label}
            >
                <span class="icon">{tool.icon}</span>
            </button>
        {/each}
    </div>

    <div class="divider"></div>

    <!-- Color swatches: 2-column grid -->
    <div class="section colors">
        <div class="color-grid">
            {#each colors as color}
                <button
                    class="color-btn"
                    class:active={toolStore.currentColor === color}
                    style="background-color: {color};"
                    onclick={() => {
                        toolStore.setColor(color);
                        if (
                            toolStore.selectedAnnotationId &&
                            tabStore.activeTabId
                        ) {
                            tabStore.updateAnnotation(
                                tabStore.activeTabId,
                                toolStore.selectedAnnotationId,
                                { color: color },
                            );
                            tabStore.commitHistory(tabStore.activeTabId);
                        }
                    }}
                    aria-label={color}
                ></button>
            {/each}
        </div>
        <input
            type="color"
            class="color-picker-full"
            value={toolStore.currentColor}
            oninput={(e) => {
                const val = e.currentTarget.value;
                toolStore.setColor(val);
                if (toolStore.selectedAnnotationId && tabStore.activeTabId) {
                    tabStore.updateAnnotation(
                        tabStore.activeTabId,
                        toolStore.selectedAnnotationId,
                        { color: val },
                    );
                }
            }}
            onchange={() => {
                if (toolStore.selectedAnnotationId && tabStore.activeTabId) {
                    tabStore.commitHistory(tabStore.activeTabId);
                }
            }}
        />
    </div>

    <div class="divider"></div>

    <!-- Sliders section -->
    <div class="section stroke">
        {#if isSpeechBubble}
            <!-- Speech bubble: pointer width + font size side by side -->
            <div class="slider-pair">
                <div class="slider-col">
                    <div class="stroke-label">Ptr: {toolStore.currentPointerWidth}</div>
                    <input
                        type="range"
                        min="4"
                        max="60"
                        step="1"
                        value={toolStore.currentPointerWidth}
                        oninput={(e) => {
                            const val = parseInt(e.currentTarget.value);
                            toolStore.setPointerWidth(val);
                            updateSpeechBubbleData("pointerWidth", val);
                        }}
                        onchange={commitChange}
                        class="stroke-slider vertical"
                    />
                </div>
                <div class="slider-col">
                    <div class="stroke-label">Font: {toolStore.currentFontSize}</div>
                    <input
                        type="range"
                        min="10"
                        max="60"
                        step="1"
                        value={toolStore.currentFontSize}
                        oninput={(e) => {
                            const val = parseInt(e.currentTarget.value);
                            toolStore.setFontSize(val);
                            updateSpeechBubbleData("fontSize", val);
                        }}
                        onchange={commitChange}
                        class="stroke-slider vertical"
                    />
                </div>
            </div>
        {:else}
            <!-- Default: single size slider -->
            <div class="stroke-label">
                {#if toolStore.currentTool === "pixelate"}
                    Density: {toolStore.currentStrokeWidth}
                {:else}
                    Size: {toolStore.currentStrokeWidth}px
                {/if}
            </div>
            <input
                type="range"
                min="1"
                max={toolStore.currentTool === "pixelate" ? 50 : 20}
                step="1"
                value={toolStore.currentStrokeWidth}
                oninput={(e) => {
                    const val = parseInt(e.currentTarget.value);
                    toolStore.setStrokeWidth(val);
                    if (
                        toolStore.selectedAnnotationId &&
                        tabStore.activeTabId
                    ) {
                        tabStore.updateAnnotation(
                            tabStore.activeTabId,
                            toolStore.selectedAnnotationId,
                            { strokeWidth: val },
                        );
                    }
                }}
                onchange={() => {
                    if (
                        toolStore.selectedAnnotationId &&
                        tabStore.activeTabId
                    ) {
                        tabStore.commitHistory(tabStore.activeTabId);
                    }
                }}
                class="stroke-slider vertical"
            />
        {/if}
    </div>
</div>

{#if showSettings}
    <SettingsModal onClose={() => (showSettings = false)} />
{/if}

<style>
    .palette {
        display: flex;
        flex-direction: column;
        background: var(--bg-glass);
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
        padding: 8px;
        border: 1px solid var(--border);
        border-radius: var(--radius-md);
        box-shadow: var(--shadow-lg);
        gap: 12px;
        width: var(--palette-width);
        box-sizing: border-box; /* --palette-width is the reserved outer width */
        align-items: center;
        user-select: none;
        height: auto;
        max-height: 90vh;
        overflow-y: auto;

        /* Floating positioning */
        position: fixed;
        left: var(--palette-offset);
        top: 50%;
        transform: translateY(-50%);
        z-index: 100;
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 6px;
        width: 100%;
        align-items: center;
    }

    /* Utility buttons: centered stack with paired rows */
    .utility-section {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    }

    .utility-pair {
        display: flex;
        gap: 4px;
    }

    /* Tools: 2-column grid */
    .tools-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 4px;
        justify-items: center;
    }

    .tool-btn {
        width: 32px;
        height: 32px;
        border-radius: var(--radius-sm);
        border: 1px solid transparent;
        background: transparent;
        color: var(--text-muted);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 18px;
        transition: all 0.2s;
    }

    .tool-btn:hover {
        background: var(--bg-secondary);
        color: var(--text-main);
    }

    .tool-btn.active {
        background: var(--accent);
        color: var(--bg-primary);
        border-color: transparent;
    }

    .icon.save-as {
        position: relative;
    }

    .icon.save-as .plus {
        position: absolute;
        bottom: -2px;
        right: -4px;
        font-size: 12px;
        font-weight: 700;
        color: #e53935;
        line-height: 1;
    }

    .divider {
        height: 1px;
        width: 100%;
        background: var(--border);
        flex-shrink: 0;
    }

    /* Color swatches: 2-column grid */
    .color-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        gap: 6px;
        justify-items: center;
    }

    .color-btn {
        width: 28px;
        height: 28px;
        border-radius: 50%;
        border: 2px solid var(--border);
        cursor: pointer;
        padding: 0;
        transition: transform 0.2s;
    }

    .color-btn:hover {
        transform: scale(1.1);
        border-color: var(--text-main);
    }

    .color-btn.active {
        border-color: var(--text-main);
        box-shadow: 0 0 0 2px var(--accent);
    }

    .color-picker-full {
        width: 100%;
        height: 28px;
        padding: 0;
        border: 1px solid var(--border);
        border-radius: var(--radius-sm);
        background: none;
        cursor: pointer;
    }

    /* Sliders */
    .slider-pair {
        display: flex;
        gap: 8px;
        width: 100%;
        justify-content: center;
    }

    .slider-col {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 4px;
    }

    .stroke-label {
        color: var(--text-muted);
        font-size: 10px;
        text-align: center;
    }

    .stroke-slider {
        width: 100%;
        cursor: pointer;
        accent-color: var(--accent);
    }

    .stroke-slider.vertical {
        writing-mode: bt-lr;
        -webkit-appearance: slider-vertical;
        appearance: slider-vertical;
        width: 8px;
        height: 100px;
        padding: 0 5px;
    }

    .palette::-webkit-scrollbar {
        width: 4px;
    }
    .palette::-webkit-scrollbar-thumb {
        background: var(--border);
        border-radius: 2px;
    }
</style>
