import { browser } from "$app/environment";

// Default Settings
export const DEFAULT_SETTINGS = {
    defaultTool: "pen",
    defaultStrokeWidth: 2,
    defaultPixelDensity: 10,
    defaultColor: "#ff0000",
    toolbarPalette: [
        "#ff0000", "#00ff00", "#0000ff", "#ffff00",
        "#ff00ff", "#00ffff", "#000000", "#ffffff"
    ],
    storagePath: ""
};

class SettingsStore {
    // State using Svelte 5 runes
    settings = $state({ ...DEFAULT_SETTINGS });

    constructor() {
        if (browser) {
            this.load();
        }
    }

    load() {
        try {
            const stored = localStorage.getItem("crosshair_settings");
            if (stored) {
                const parsed = JSON.parse(stored);
                this.settings = { ...this.settings, ...parsed };

                // Validate palette length (migration safety)
                if (!Array.isArray(this.settings.toolbarPalette) || this.settings.toolbarPalette.length !== 8) {
                    this.settings.toolbarPalette = [...DEFAULT_SETTINGS.toolbarPalette];
                }
            }
        } catch (e) {
            console.error("Failed to load settings", e);
        }
    }

    save() {
        try {
            // Determine what changed using dependency tracking
            // But simplified: just save the whole object
            // Note: In Svelte 5 class, $effect runs when dependencies change.
            // referencing this.settings inside effect registers dependency.
            const json = JSON.stringify(this.settings);
            localStorage.setItem("crosshair_settings", json);
        } catch (e) {
            console.error("Failed to save settings", e);
        }
    }

    // Actions
    updateSetting(key: keyof typeof DEFAULT_SETTINGS, value: any) {
        // @ts-ignore - dynamic key assignment
        this.settings[key] = value;
    }

    updatePaletteColor(index: number, color: string) {
        if (index >= 0 && index < this.settings.toolbarPalette.length) {
            this.settings.toolbarPalette[index] = color;
        }
    }
}

export const settingsStore = new SettingsStore();
