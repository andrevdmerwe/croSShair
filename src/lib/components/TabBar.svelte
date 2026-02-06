<script lang="ts">
  import { tabStore } from "../stores/tabs.svelte";

  function selectTab(id: string) {
    tabStore.activeTabId = id;
  }

  function closeTab(e: MouseEvent, id: string) {
    e.stopPropagation();
    // In real implementation we'll check for dirty state here
    tabStore.removeTab(id);
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
</div>

<style>
  .tab-bar {
    display: flex;
    background: var(--bg-secondary);
    padding: 4px 4px 0 4px; /* Slight adjust */
    height: 40px;
    align-items: flex-end;
    overflow-x: auto;
    border-bottom: 1px solid var(--border);
    user-select: none;
    width: 100%;
    /* Drag region */
    app-region: drag; /* Standard CSS property for some webviews */
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
</style>
