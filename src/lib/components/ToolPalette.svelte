<script lang="ts">
    import { toolStore } from "$lib/stores/tools.svelte";
    import { tabStore } from "$lib/stores/tabs.svelte";
    import { settingsStore } from "$lib/stores/settings.svelte";
    import SettingsModal from "$lib/components/SettingsModal.svelte";
    import type { Tool } from "$lib/types";
    import { onMount } from "svelte";

    const tools: { id: Tool; label: string; icon: string }[] = [
        // Select tool removed as per request
        { id: "pen", label: "Pen", icon: "✎" },
        { id: "highlighter", label: "Highlighter", icon: "🖊" },
        { id: "text", label: "Text", icon: "T" },
        { id: "rectangle", label: "Rectangle", icon: "☐" },
        { id: "ellipse", label: "Ellipse", icon: "◯" },
        { id: "line", label: "Line", icon: "／" },
        { id: "arrow", label: "Arrow", icon: "↗" },
        { id: "step", label: "Step Number", icon: "➊" },
        { id: "pixelate", label: "Pixelate", icon: "░" },
    ];

    // Dynamic palette from settings
    let colors = $derived(settingsStore.settings.toolbarPalette);
    let showSettings = $state(false);

    let { onCapture, onSave, onCopy } = $props<{
        onCapture?: () => void;
        onSave?: () => void;
        onCopy?: () => void;
    }>();

    onMount(() => {
        // Initialize tool defaults from settings on startup
        toolStore.setTool(settingsStore.settings.defaultTool as Tool);
        toolStore.setStrokeWidth(settingsStore.settings.defaultStrokeWidth);
        toolStore.setColor(settingsStore.settings.defaultColor);
    });

    function commitChange() {
        console.log("ToolPalette: commitChange called", {
            activeTab: tabStore.activeTabId,
            selected: toolStore.selectedAnnotationId,
        });
        if (tabStore.activeTabId && toolStore.selectedAnnotationId) {
            tabStore.commitHistory(tabStore.activeTabId);
        }
    }
</script>

<div class="palette">
    <!-- Utility Buttons (Top) -->
    <div class="section settings">
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

        <div class="divider"></div>

        <button class="tool-btn" onclick={() => onSave?.()} title="Save Image">
            <span class="icon">💾</span>
        </button>

        <button
            class="tool-btn"
            onclick={() => onCopy?.()}
            title="Copy to Clipboard"
        >
            <span class="icon">📋</span>
        </button>

        <div class="divider"></div>

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

    <div class="divider"></div>

    <div class="section tools">
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

    <div class="section colors">
        {#each colors as color}
            <button
                class="color-btn"
                class:active={toolStore.currentTool !== "highlighter" &&
                    toolStore.currentColor === color}
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
        <input
            type="color"
            class="color-picker"
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

    <div class="section stroke">
        <div class="stroke-label">
            {#if toolStore.currentTool === "pixelate"}
                Density: {toolStore.currentStrokeWidth}
            {:else}
                Size: {toolStore.currentStrokeWidth}px
            {/if}
        </div>
        <div class="stroke-options">
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
        </div>
    </div>
</div>

{#if showSettings}
    <SettingsModal onClose={() => (showSettings = false)} />
{/if}

<style>
    .palette {
        display: flex;
        flex-direction: column;
        background: #2a2a2a;
        padding: 8px;
        border-right: 1px solid #444;
        gap: 16px;
        width: 48px;
        align-items: center;
        user-select: none;
        height: 100%; /* Ensure full height */
        overflow-y: auto; /* Scroll if needed on small screens */
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 8px;
        width: 100%;
        align-items: center;
    }

    .tool-btn {
        width: 32px;
        height: 32px;
        border-radius: 4px;
        border: 1px solid transparent;
        background: transparent;
        color: #ccc;
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        font-size: 18px;
        transition: all 0.2s;
    }

    .tool-btn:hover {
        background: #444;
    }

    .tool-btn.active {
        background: #4a90e2;
        color: white;
        border-color: #357abd;
    }

    .divider {
        height: 1px;
        width: 100%;
        background: #444;
        flex-shrink: 0;
    }

    .color-btn {
        width: 24px;
        height: 24px;
        border-radius: 50%;
        border: 2px solid transparent;
        cursor: pointer;
        padding: 0;
    }

    .color-btn:hover {
        transform: scale(1.1);
    }

    .color-btn.active {
        border-color: white;
        box-shadow: 0 0 0 2px #4a90e2;
    }

    .color-picker {
        width: 24px;
        height: 24px;
        padding: 0;
        border: none;
        background: none;
        cursor: pointer;
    }

    .stroke-options {
        display: flex;
        flex-direction: column;
        gap: 4px;
    }

    .stroke-label {
        color: #888;
        font-size: 10px;
        text-align: center;
    }

    .stroke-slider {
        width: 100%;
        cursor: pointer;
    }

    .stroke-slider.vertical {
        writing-mode: bt-lr; /* IE */
        -webkit-appearance: slider-vertical; /* WebKit */
        width: 8px;
        height: 100px; /* Adjust height as needed */
        padding: 0 5px;
    }

    /* Settings button specific */
    .section.settings {
        margin-bottom: 0; /* Keep it tight to top */
    }
</style>
