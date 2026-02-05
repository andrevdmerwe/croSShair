<script lang="ts">
    import {
        settingsStore,
        DEFAULT_SETTINGS,
    } from "$lib/stores/settings.svelte";
    import { invoke } from "@tauri-apps/api/core";
    import { open } from "@tauri-apps/plugin-dialog";

    let { onClose } = $props<{ onClose: () => void }>();

    // Tools list matching ToolPalette
    const tools = [
        { id: "pen", label: "Pen" },
        { id: "highlighter", label: "Highlighter" },
        { id: "rectangle", label: "Rectangle" },
        { id: "ellipse", label: "Ellipse" },
        { id: "line", label: "Line" },
        { id: "arrow", label: "Arrow" },
        { id: "step", label: "Step Number" },
        { id: "pixelate", label: "Pixelate" },
    ];

    async function selectStorageFolder() {
        try {
            const selected = await open({
                directory: true,
                multiple: false,
                defaultPath: settingsStore.settings.storagePath || undefined,
            });

            if (selected && !Array.isArray(selected)) {
                settingsStore.updateSetting("storagePath", selected);
            }
        } catch (e) {
            console.error("Failed to open dialog", e);
        }
    }
</script>

<div class="modal-backdrop" onclick={onClose}>
    <div class="modal-content" onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
            <h2>Settings</h2>
            <button class="close-btn" onclick={onClose}>&times;</button>
        </div>

        <div class="modal-body">
            <!-- Defaults Section -->
            <section>
                <h3>Defaults</h3>

                <div class="setting-row">
                    <label for="default-tool">Default Tool</label>
                    <select
                        id="default-tool"
                        value={settingsStore.settings.defaultTool}
                        onchange={(e) =>
                            settingsStore.updateSetting(
                                "defaultTool",
                                e.currentTarget.value,
                            )}
                    >
                        {#each tools as tool}
                            <option value={tool.id}>{tool.label}</option>
                        {/each}
                    </select>
                </div>

                <div class="setting-row">
                    <label for="default-width"
                        >Default Line Width ({settingsStore.settings
                            .defaultStrokeWidth}px)</label
                    >
                    <input
                        type="range"
                        id="default-width"
                        min="1"
                        max="20"
                        value={settingsStore.settings.defaultStrokeWidth}
                        oninput={(e) =>
                            settingsStore.updateSetting(
                                "defaultStrokeWidth",
                                parseInt(e.currentTarget.value),
                            )}
                    />
                </div>

                <div class="setting-row">
                    <label for="default-color">Default Color</label>
                    <div class="color-picker-wrapper">
                        <input
                            type="color"
                            id="default-color"
                            value={settingsStore.settings.defaultColor}
                            oninput={(e) =>
                                settingsStore.updateSetting(
                                    "defaultColor",
                                    e.currentTarget.value,
                                )}
                        />
                        <span>{settingsStore.settings.defaultColor}</span>
                    </div>
                </div>

                <div class="setting-row">
                    <label for="default-density"
                        >Default Pixel Density ({settingsStore.settings
                            .defaultPixelDensity})</label
                    >
                    <input
                        type="range"
                        id="default-density"
                        min="1"
                        max="50"
                        value={settingsStore.settings.defaultPixelDensity}
                        oninput={(e) =>
                            settingsStore.updateSetting(
                                "defaultPixelDensity",
                                parseInt(e.currentTarget.value),
                            )}
                    />
                </div>
            </section>

            <hr />

            <!-- Palette Section -->
            <section>
                <h3>Toolbar Palette</h3>
                <p class="hint">
                    Customize the 8 colors available in the toolbar.
                </p>
                <div class="palette-grid">
                    {#each settingsStore.settings.toolbarPalette as color, i}
                        <input
                            type="color"
                            value={color}
                            oninput={(e) =>
                                settingsStore.updatePaletteColor(
                                    i,
                                    e.currentTarget.value,
                                )}
                            title={`Color slot ${i + 1}`}
                        />
                    {/each}
                </div>
            </section>

            <hr />

            <!-- Storage Section -->
            <section>
                <h3>Storage</h3>
                <div class="setting-row">
                    <label for="storage-path">Screenshot Save Location</label>
                    <div class="path-selector">
                        <input
                            type="text"
                            id="storage-path"
                            value={settingsStore.settings.storagePath ||
                                "Default (Pictures/Screenshots)"}
                            readonly
                        />
                        <button class="browse-btn" onclick={selectStorageFolder}
                            >Browse...</button
                        >
                    </div>
                </div>
            </section>
        </div>
    </div>
</div>

<style>
    .modal-backdrop {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
        z-index: 1000;
        backdrop-filter: blur(2px);
    }

    .modal-content {
        background: #2a2a2a;
        width: 500px;
        max-width: 90vw;
        max-height: 85vh;
        border-radius: 8px;
        box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        color: #eee;
        border: 1px solid #444;
    }

    .modal-header {
        padding: 16px;
        border-bottom: 1px solid #444;
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: #333;
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.2rem;
    }

    .close-btn {
        background: none;
        border: none;
        color: #aaa;
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0 8px;
    }
    .close-btn:hover {
        color: white;
    }

    .modal-body {
        padding: 16px;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    section h3 {
        margin-top: 0;
        font-size: 1rem;
        color: #4a90e2;
        margin-bottom: 12px;
    }

    .setting-row {
        display: flex;
        flex-direction: column;
        gap: 6px;
        margin-bottom: 12px;
    }

    label {
        font-size: 0.9rem;
        color: #ccc;
    }

    select,
    input[type="text"] {
        background-color: #1a1a1a;
        border: 1px solid #444;
        color: white;
        padding: 8px;
        border-radius: 4px;
    }

    /* Custom select styling to override system defaults */
    select {
        appearance: none;
        -webkit-appearance: none;
        background-image: url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23FFFFFF%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E");
        background-repeat: no-repeat;
        background-position: right 0.7em top 50%;
        background-size: 0.65em auto;
        padding-right: 2em;
    }

    option {
        background-color: #1a1a1a;
        color: white;
    }

    .color-picker-wrapper {
        display: flex;
        align-items: center;
        gap: 8px;
    }

    input[type="color"] {
        background: none;
        border: none;
        width: 40px;
        height: 40px;
        cursor: pointer;
    }

    .palette-grid {
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        gap: 8px;
    }

    .palette-grid input[type="color"] {
        width: 100%;
        height: 32px;
        background: #1a1a1a;
        padding: 2px;
        border-radius: 4px;
    }

    .path-selector {
        display: flex;
        gap: 8px;
    }

    .path-selector input {
        flex: 1;
        color: #888;
    }

    .browse-btn {
        padding: 8px 16px;
        background: #444;
        border: none;
        color: white;
        border-radius: 4px;
        cursor: pointer;
    }
    .browse-btn:hover {
        background: #555;
    }

    hr {
        border: 0;
        border-top: 1px solid #444;
        width: 100%;
        margin: 8px 0;
    }

    .hint {
        font-size: 0.8rem;
        color: #888;
        margin-top: -8px;
        margin-bottom: 8px;
    }
</style>
