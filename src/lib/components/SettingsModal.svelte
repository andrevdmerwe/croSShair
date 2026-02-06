<script lang="ts">
    import {
        settingsStore,
        DEFAULT_SETTINGS,
    } from "$lib/stores/settings.svelte";
    import { themeStore, THEMES } from "$lib/stores/theme.svelte";
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
            <!-- Appearance Section -->
            <section>
                <h3>Appearance</h3>

                <div class="setting-row">
                    <label for="theme-select">Theme</label>
                    <select
                        id="theme-select"
                        value={themeStore.currentTheme}
                        onchange={(e) =>
                            themeStore.setTheme(e.currentTarget.value)}
                    >
                        {#each THEMES as theme}
                            <option value={theme.value}>{theme.label}</option>
                        {/each}
                    </select>
                </div>

                <!-- Custom Overrides -->
                <div class="setting-group">
                    <label>Theme Overrides</label>
                    <div class="color-overrides">
                        <div class="color-field">
                            <span>Accent</span>
                            <input
                                type="color"
                                value={themeStore.customOverrides["accent"] ||
                                    ""}
                                oninput={(e) =>
                                    themeStore.updateOverride(
                                        "accent",
                                        e.currentTarget.value,
                                    )}
                            />
                            {#if themeStore.customOverrides["accent"]}
                                <button
                                    class="reset-btn"
                                    onclick={() =>
                                        themeStore.updateOverride("accent", "")}
                                    title="Reset">↺</button
                                >
                            {/if}
                        </div>
                        <div class="color-field">
                            <span>Background</span>
                            <input
                                type="color"
                                value={themeStore.customOverrides[
                                    "bg-primary"
                                ] || ""}
                                oninput={(e) =>
                                    themeStore.updateOverride(
                                        "bg-primary",
                                        e.currentTarget.value,
                                    )}
                            />
                            {#if themeStore.customOverrides["bg-primary"]}
                                <button
                                    class="reset-btn"
                                    onclick={() =>
                                        themeStore.updateOverride(
                                            "bg-primary",
                                            "",
                                        )}
                                    title="Reset">↺</button
                                >
                            {/if}
                        </div>
                    </div>
                </div>
            </section>

            <hr />

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
                    <div class="range-wrapper">
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
        background: var(--bg-primary);
        width: 500px;
        max-width: 90vw;
        max-height: 85vh;
        border-radius: var(--radius-lg);
        box-shadow: var(--shadow-lg);
        display: flex;
        flex-direction: column;
        overflow: hidden;
        color: var(--text-main);
        border: 1px solid var(--border);
    }

    .modal-header {
        padding: 16px;
        border-bottom: 1px solid var(--border);
        display: flex;
        justify-content: space-between;
        align-items: center;
        background: var(--bg-secondary);
    }

    .modal-header h2 {
        margin: 0;
        font-size: 1.2rem;
        font-weight: 600;
    }

    .close-btn {
        background: none;
        border: none;
        color: var(--text-muted);
        font-size: 1.5rem;
        cursor: pointer;
        padding: 0 8px;
        line-height: 1;
        transition: color 0.2s;
    }
    .close-btn:hover {
        color: var(--text-main);
    }

    .modal-body {
        padding: 20px;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 20px;
    }

    section h3 {
        margin-top: 0;
        font-size: 1rem;
        color: var(--accent);
        margin-bottom: 12px;
        border-bottom: 1px solid var(--border);
        padding-bottom: 4px;
        display: inline-block;
    }

    .setting-row {
        display: flex;
        flex-direction: column;
        gap: 8px;
        margin-bottom: 12px;
    }

    label {
        font-size: 0.9rem;
        color: var(--text-muted);
        font-weight: 500;
    }

    select,
    input[type="text"] {
        background-color: var(--bg-secondary);
        border: 1px solid var(--border);
        color: var(--text-main);
        padding: 8px 12px;
        border-radius: var(--radius-sm);
        outline: none;
        transition: border-color 0.2s;
    }

    select:focus,
    input[type="text"]:focus {
        border-color: var(--accent);
    }

    /* Custom select styling */
    select {
        appearance: none;
        -webkit-appearance: none;
        /* Using a simplified arrow or removing it to rely on system if complex image fails */
        /* Reusing the previous data URI but simpler logic */
        padding-right: 2em;
        cursor: pointer;
    }

    option {
        background-color: var(--bg-secondary);
        color: var(--text-main);
    }

    .setting-group {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .color-overrides {
        display: flex;
        gap: 16px;
    }

    .color-field {
        display: flex;
        align-items: center;
        gap: 8px;
        font-size: 0.85rem;
        color: var(--text-muted);
    }

    .reset-btn {
        background: none;
        border: none;
        color: var(--text-muted);
        cursor: pointer;
        font-size: 1.1em;
        padding: 0 4px;
    }
    .reset-btn:hover {
        color: var(--accent);
    }

    .color-picker-wrapper {
        display: flex;
        align-items: center;
        gap: 12px;
    }

    input[type="color"] {
        background: none;
        border: none;
        width: 32px;
        height: 32px;
        cursor: pointer;
        padding: 0;
        border-radius: 4px; /* Some browsers support this on wrapper */
    }

    /* Range slider styling */
    input[type="range"] {
        width: 100%;
        accent-color: var(--accent);
    }

    .palette-grid {
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        gap: 8px;
    }

    .palette-grid input[type="color"] {
        width: 100%;
        height: 36px;
        background: var(--bg-secondary);
        padding: 2px;
        border-radius: var(--radius-sm);
        border: 1px solid var(--border);
    }

    .path-selector {
        display: flex;
        gap: 8px;
    }

    .path-selector input {
        flex: 1;
        color: var(--text-muted);
        /* Read only style */
        cursor: default;
    }

    .browse-btn {
        padding: 8px 16px;
        background: var(--bg-secondary);
        border: 1px solid var(--border);
        color: var(--text-main);
        border-radius: var(--radius-sm);
        cursor: pointer;
        transition: all 0.2s;
    }
    .browse-btn:hover {
        background: var(
            --bg-primary
        ); /* Darker/Lighter depending on theme? Usually hover should be distinct */
        border-color: var(--accent);
    }

    hr {
        border: 0;
        border-top: 1px solid var(--border);
        width: 100%;
        margin: 8px 0;
    }

    .hint {
        font-size: 0.8rem;
        color: var(--text-muted);
        margin-top: -8px;
        margin-bottom: 12px;
    }

    /* Scrollbar */
    .modal-body::-webkit-scrollbar {
        width: 6px;
    }
    .modal-body::-webkit-scrollbar-track {
        background: transparent;
    }
    .modal-body::-webkit-scrollbar-thumb {
        background: var(--border);
        border-radius: 3px;
    }
    .modal-body::-webkit-scrollbar-thumb:hover {
        background: var(--text-muted);
    }
</style>
