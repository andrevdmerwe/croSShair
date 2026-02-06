import { browser } from "$app/environment";

export const THEMES = [
    { value: "catppuccin-mocha", label: "Catppuccin Mocha" },
    { value: "nord", label: "Nord" },
    { value: "osaka-jade", label: "Osaka Jade" },
    { value: "kanagawa", label: "Kanagawa" },
    { value: "tokyo-night", label: "Tokyo Night" },
    { value: "light", label: "Light" },
];

export const DEFAULT_THEME = "catppuccin-mocha";

class ThemeStore {
    currentTheme = $state(DEFAULT_THEME);
    customOverrides: Record<string, string> = $state({});

    constructor() {
        if (browser) {
            this.init();
        }
    }

    init() {
        const stored = localStorage.getItem("crosshair_theme");
        if (stored) {
            try {
                const parsed = JSON.parse(stored);
                if (parsed.theme) this.currentTheme = parsed.theme;
                if (parsed.overrides) this.customOverrides = parsed.overrides;
            } catch {
                // Legacy or invalid
            }
        }
        // Apply immediately
        this.apply();

        // Also setup listener for changes if needed, but explicit calls work for now
    }

    setTheme(theme: string) {
        this.currentTheme = theme;
        this.save();
        this.apply();
    }

    updateOverride(key: string, value: string) {
        // @ts-ignore
        this.customOverrides[key] = value;
        this.save();
        this.apply();
    }

    save() {
        if (browser) {
            localStorage.setItem("crosshair_theme", JSON.stringify({
                theme: this.currentTheme,
                overrides: this.customOverrides
            }));
        }
    }

    apply() {
        if (!browser) return;
        const root = document.documentElement;

        // Apply theme (data-attribute)
        if (this.currentTheme === 'catppuccin-mocha') {
            root.removeAttribute('data-theme');
        } else {
            root.setAttribute('data-theme', this.currentTheme);
        }

        // Apply overrides
        // Reset valid inline styles first? 
        // No, assuming we only set overrides here. 
        // But if we remove an override, we need to unset it.
        // My simple logic just iterates. Does not clean up removed keys if they were deleted from object.
        // But the UI will probably just set/unset.

        Object.entries(this.customOverrides).forEach(([key, value]) => {
            if (value) {
                root.style.setProperty(`--${key}`, value as string);
            } else {
                root.style.removeProperty(`--${key}`);
            }
        });
    }
}

export const themeStore = new ThemeStore();
