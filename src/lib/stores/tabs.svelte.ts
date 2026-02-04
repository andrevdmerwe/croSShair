import type { TabData } from '../types';

class TabStore {
    tabs = $state<TabData[]>([]);
    activeTabId = $state<string | null>(null);

    addTab(tab: TabData) {
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
    }

    updateTab(id: string, updates: Partial<TabData>) {
        const index = this.tabs.findIndex(t => t.id === id);
        if (index !== -1) {
            this.tabs[index] = { ...this.tabs[index], ...updates };
        }
    }

    // For getting a tab by ID
    getTab(id: string) {
        return this.tabs.find(t => t.id === id);
    }
}

export const tabStore = new TabStore();
