<script lang="ts">
    import { toolStore } from "$lib/stores/tools.svelte";
    import { tabStore } from "$lib/stores/tabs.svelte";
    import type { Tool } from "$lib/types";

    const tools: { id: Tool; label: string; icon: string }[] = [
        // Select tool removed as per request
        { id: "pen", label: "Pen", icon: "✎" },
        { id: "highlighter", label: "Highlighter", icon: "🖊" },
        { id: "rectangle", label: "Rectangle", icon: "☐" },
        { id: "ellipse", label: "Ellipse", icon: "◯" },
        { id: "line", label: "Line", icon: "／" },
        { id: "arrow", label: "Arrow", icon: "↗" },
        { id: "step", label: "Step Number", icon: "➊" },
        { id: "pixelate", label: "Pixelate", icon: "░" },
    ];

    // Assuming 'colors' array was defined here previously,
    // it's not provided in the instruction's script block,
    // so we'll keep the original definition if it existed,
    // or assume it's part of the '...' if not explicitly defined.
    // For now, we'll keep the original structure for 'colors' if it was outside the script block,
    // or assume it's part of the '...' if it was inside.
    // Given the HTML uses `{#each colors as color}`, `colors` must be an array.
    // Let's define a placeholder for colors based on common usage.
    const colors = [
        "#FF0000",
        "#00FF00",
        "#0000FF",
        "#FFFF00",
        "#00FFFF",
        "#FF00FF",
        "#000000",
        "#FFFFFF",
    ];

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

```svelte
<div class="palette">
    <div class="section tools">
        {#each tools as tool}
            <button
                class="tool-btn"
                class:active={toolStore.currentTool === tool.id}
                onclick={() => toolStore.setTool(tool.id)}
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
                class:active={toolStore.currentColor === color}
                style="background-color: {color};"
                onclick={() => {
                    toolStore.setColor(color);
                    // Explicitly update if something is selected to ensure order: Update -> Commit
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
                // Continuous update for preview
                if (toolStore.selectedAnnotationId && tabStore.activeTabId) {
                    tabStore.updateAnnotation(
                        tabStore.activeTabId,
                        toolStore.selectedAnnotationId,
                        { color: val },
                    );
                }
            }}
            onchange={() => {
                // Commit on release/close
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
                    // Continuous update for preview
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
                    // Commit on release
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
</style>
