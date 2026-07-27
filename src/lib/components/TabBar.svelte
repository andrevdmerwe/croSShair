<script lang="ts">
  import { tabStore } from "../stores/tabs.svelte";
  import { ask } from "@tauri-apps/plugin-dialog";

  let { onSaveTab } = $props<{
    onSaveTab?: (tabId: string) => Promise<void>;
  }>();

  function selectTab(id: string) {
    tabStore.activeTabId = id;
  }

  async function closeTab(e: MouseEvent, id: string) {
    e.stopPropagation();
    const tab = tabStore.getTab(id);

    if (tab?.isDirty) {
      const shouldSave = await ask(
        `"${tab.label}" has unsaved changes. Save before closing?`,
        {
          title: "Unsaved Changes",
          kind: "warning",
          okLabel: "Save",
          cancelLabel: "Don't Save",
        },
      );

      if (shouldSave && onSaveTab) {
        await onSaveTab(id);
      }
    }

    tabStore.removeTab(id);
  }

  async function closeAllTabs() {
    // Check for any dirty tabs
    const dirtyTabs = tabStore.tabs.filter((t) => t.isDirty);

    if (dirtyTabs.length > 0) {
      const shouldSave = await ask(
        `${dirtyTabs.length} tab(s) have unsaved changes. Save all before closing?`,
        {
          title: "Unsaved Changes",
          kind: "warning",
          okLabel: "Save All",
          cancelLabel: "Don't Save",
        },
      );

      if (shouldSave && onSaveTab) {
        for (const tab of dirtyTabs) {
          await onSaveTab(tab.id);
        }
      }
    }

    // Close all tabs
    const tabIds = tabStore.tabs.map((t) => t.id);
    tabIds.forEach((id) => tabStore.removeTab(id));
  }
</script>

<div class="tab-bar" data-tauri-drag-region>
  {#each tabStore.tabs as tab}
    <button
      class="tab"
      class:active={tab.id === tabStore.activeTabId}
      onclick={() => selectTab(tab.id)}
    >
      <span class="label">{tab.label}</span>
      <span
        class="close-btn"
        role="button"
        tabindex="0"
        onclick={(e) => closeTab(e, tab.id)}
        onkeydown={(e) =>
          e.key === "Enter" && closeTab(e as unknown as MouseEvent, tab.id)}
      >
        ×
      </span>
    </button>
  {/each}
  {#if tabStore.tabs.length > 1}
    <button class="close-all-btn" onclick={closeAllTabs} title="Close all tabs">
      ✕ All
    </button>
  {/if}
</div>

<style>
  .tab-bar {
    display: flex;
    background: var(--bg-secondary);
    padding: 4px 4px 0 4px; /* Slight adjust */
    height: 48px;
    align-items: flex-end;
    overflow-x: auto;
    border-bottom: 1px solid var(--border);
    user-select: none;
    width: 100%;
    /* Drag region */
    app-region: drag; /* Standard CSS property for some webviews */
  }

  /* Custom scrollbar: fixed thickness, reserved below the tabs instead of
     overlaying them, and does not grow/hide on hover like the OS default. */
  .tab-bar::-webkit-scrollbar {
    height: 6px;
  }

  .tab-bar::-webkit-scrollbar-track {
    background: transparent;
  }

  .tab-bar::-webkit-scrollbar-thumb {
    background: var(--border);
    border-radius: 3px;
  }

  .tab-bar::-webkit-scrollbar-thumb:hover {
    background: var(--text-muted);
  }

  .tab {
    display: flex;
    align-items: center;
    padding: 8px 16px;
    margin-right: 2px;
    background: var(--bg-primary);
    border: 1px solid var(--border);
    border-bottom: none;
    border-radius: var(--radius-md) var(--radius-md) 0 0;
    cursor: pointer;
    font-size: 13px;
    color: var(--text-muted);
    max-width: 200px;
    min-width: 100px;
    height: 32px;
    transition: all 0.2s;
    /* Non-draggable interactive elements need no-drag */
    -webkit-app-region: no-drag;
  }

  .tab:hover {
    background: var(--bg-secondary);
    color: var(--text-main);
  }

  .tab.active {
    background: var(--bg-primary);
    border-color: var(--accent);
    border-bottom: 1px solid var(--bg-primary); /* Blend with content */
    color: var(--text-main);
    font-weight: 500;
    z-index: 10;
    margin-bottom: -1px;
    height: 33px;
    border-top: 2px solid var(--accent); /* Highlight active tab */
  }

  .label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-right: 8px;
  }

  .close-btn {
    width: 18px;
    height: 18px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    font-size: 14px;
    line-height: 1;
    color: var(--text-muted);
    transition: all 0.2s;
  }

  .close-btn:hover {
    background: var(--bg-glass);
    color: var(--accent);
  }

  .close-all-btn {
    display: flex;
    align-items: center;
    padding: 6px 12px;
    margin-left: auto;
    background: var(--bg-glass);
    border: 1px solid var(--border);
    border-radius: var(--radius-md);
    cursor: pointer;
    font-size: 12px;
    color: var(--text-muted);
    transition: all 0.2s;
    -webkit-app-region: no-drag;
    height: 28px;
    margin-bottom: 4px;
  }

  .close-all-btn:hover {
    background: var(--accent);
    color: white;
    border-color: var(--accent);
  }
</style>
