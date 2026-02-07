import type { Tool } from '../types';
import { settingsStore } from "./settings.svelte";

class ToolStore {
    /** Currently active tool */
    currentTool = $state<Tool>('select');

    /** Current drawing color (hex) */
    currentColor = $state<string>('#ff0000');

    /** Current stroke width in pixels */
    currentStrokeWidth = $state<number>(3);

    /** Speech bubble pointer base width (full width in pixels) */
    currentPointerWidth = $state<number>(12);

    /** Speech bubble font size in pixels */
    currentFontSize = $state<number>(26);

    /** ID of the currently selected annotation (when using select tool) */
    selectedAnnotationId = $state<string | null>(null);

    setTool(tool: Tool) {
        this.currentTool = tool;
        // Clear selection when switching tools
        if (tool !== 'select') {
            this.selectedAnnotationId = null;
        }
    }

    setColor(color: string) {
        this.currentColor = color;
    }

    setStrokeWidth(width: number) {
        this.currentStrokeWidth = width;
    }

    setPointerWidth(width: number) {
        this.currentPointerWidth = width;
    }

    setFontSize(size: number) {
        this.currentFontSize = size;
    }

    selectAnnotation(id: string | null) {
        this.selectedAnnotationId = id;
    }

    clearSelection() {
        this.selectedAnnotationId = null;
    }

    resetToDefaults() {
        // Use settings from store
        const settings = settingsStore.settings;
        this.currentTool = settings.defaultTool as Tool;
        this.currentColor = settings.defaultColor;
        this.currentStrokeWidth = settings.defaultStrokeWidth;
        this.currentPointerWidth = 12;
        this.currentFontSize = 26;
        this.selectedAnnotationId = null;
    }
}

export const toolStore = new ToolStore();
