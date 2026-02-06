import type { TabData, Annotation } from '../types';
import { toolStore } from './tools.svelte';

class TabStore {
    tabs = $state<TabData[]>([]);
    activeTabId = $state<string | null>(null);

    addTab(tab: TabData) {
        // Initialize history if needed (e.g. from existing tab data without history)
        if (!tab.history) {
            tab.history = [[]]; // Initial empty state
            tab.historyIndex = 0;
            // If tab came with annotations, that's actually the initial state
            if (tab.annotations.length > 0) {
                tab.history = [[...tab.annotations]];
            }
        }
        this.tabs.push(tab);
        this.activeTabId = tab.id;
    }

    removeTab(id: string) {
        const index = this.tabs.findIndex(t => t.id === id);
        if (index === -1) return;

        this.tabs = this.tabs.filter(t => t.id !== id);

        if (this.activeTabId === id) {
            // If closed active tab, select the one before it, or the new one at that index, or null
            const newIndex = Math.min(index, this.tabs.length - 1);
            this.activeTabId = this.tabs[newIndex]?.id ?? null;
        }

        if (this.tabs.length === 0) {
            toolStore.resetToDefaults();
        }
    }

    updateTab(id: string, updates: Partial<TabData>) {
        const index = this.tabs.findIndex(t => t.id === id);
        if (index !== -1) {
            this.tabs[index] = { ...this.tabs[index], ...updates };
        }
    }

    // --- History Management ---

    private pushHistory(tab: TabData) {
        // If we are not at the end of history, discard future states
        if (tab.historyIndex < tab.history.length - 1) {
            tab.history = tab.history.slice(0, tab.historyIndex + 1);
        }

        // Push deep copy of annotations
        const newState = JSON.parse(JSON.stringify(tab.annotations));
        tab.history.push(newState);
        tab.historyIndex++;
    }

    undo(tabId: string) {
        const tab = this.getTab(tabId);
        if (tab && tab.historyIndex > 0) {
            tab.historyIndex--;
            // Restore state
            tab.annotations = JSON.parse(JSON.stringify(tab.history[tab.historyIndex]));
        }
    }

    redo(tabId: string) {
        const tab = this.getTab(tabId);
        if (tab && tab.historyIndex < tab.history.length - 1) {
            tab.historyIndex++;
            // Restore state
            tab.annotations = JSON.parse(JSON.stringify(tab.history[tab.historyIndex]));
        }
    }

    // Annotation methods
    getAnnotation(tabId: string, annotationId: string) {
        const tab = this.getTab(tabId);
        return tab?.annotations.find(a => a.id === annotationId);
    }

    addAnnotation(tabId: string, annotation: Annotation) {
        const tab = this.getTab(tabId);
        if (tab) {
            // Create a new array reference to trigger reactivity
            tab.annotations = [...tab.annotations, annotation];

            if (annotation.type === 'step') {
                tab.stepCounter = (tab.stepCounter || 1) + 1;
            }

            tab.isDirty = true;
            this.pushHistory(tab); // Add to history
        }
    }

    updateAnnotation(tabId: string, annotationId: string, updates: any) {
        const tab = this.getTab(tabId);
        if (tab) {
            tab.annotations = tab.annotations.map(a =>
                a.id === annotationId ? { ...a, ...updates } : a
            );
            tab.isDirty = true;
            // Note: Continuous updates (like dragging) should probably NOT push history every frame
            // The Caller should handle 'committing' the history state on mouse up.
            // For now, we assume this is called on "commit" or we accept the overhead.
            // *Optimization*: The Canvas component calls this on mouse move. 
            // We should split this into "updateAnnotationPreview" and "commitAnnotationUpdate".
        }
    }

    // Optimized for dragging - does NOT push history
    updateAnnotationPreview(tabId: string, annotationId: string, updates: any) {
        const tab = this.getTab(tabId);
        if (tab) {
            tab.annotations = tab.annotations.map(a =>
                a.id === annotationId ? { ...a, ...updates } : a
            );
            // isDirty is true, but we don't push history yet
            tab.isDirty = true;
        }
    }

    // Explicitly commit current state to history (e.g. after drag end)
    commitHistory(tabId: string) {
        const tab = this.getTab(tabId);
        if (tab) {
            this.pushHistory(tab);
        }
    }

    removeAnnotation(tabId: string, annotationId: string) {
        const tab = this.getTab(tabId);
        if (tab) {
            tab.annotations = tab.annotations.filter(a => a.id !== annotationId);
            tab.isDirty = true;
            this.pushHistory(tab);
        }
    }

    // For getting a tab by ID
    getTab(id: string) {
        return this.tabs.find(t => t.id === id);
    }
}

export const tabStore = new TabStore();
