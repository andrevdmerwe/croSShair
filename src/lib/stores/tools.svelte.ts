import type { Tool } from '../types';
import { settingsStore } from "./settings.svelte";

class ToolStore {
    /** Currently active tool */
    currentTool = $state<Tool>('select');

    /** Current drawing color (hex) */
    currentColor = $state<string>('#ff0000');

    /** Current stroke width in pixels */
    currentStrokeWidth = $state<number>(3);

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
        this.selectedAnnotationId = null;
    }
}

export const toolStore = new ToolStore();
