<script lang="ts">
  import { tabStore } from '../stores/tabs.svelte';
  
  function selectTab(id: string) {
    tabStore.activeTabId = id;
  }
  
  function closeTab(e: MouseEvent, id: string) {
    e.stopPropagation();
    // In real implementation we'll check for dirty state here
    tabStore.removeTab(id);
  }
</script>

<div class="tab-bar">
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
        onkeydown={(e) => e.key === 'Enter' && closeTab(e as unknown as MouseEvent, tab.id)}
      >
        ×
      </span>
    </button>
  {/each}
</div>

<style>
  .tab-bar {
    display: flex;
    background: #e0e0e0;
    padding: 0 4px;
    height: 36px;
    align-items: flex-end;
    overflow-x: auto;
    border-bottom: 1px solid #ccc;
    user-select: none;
  }
  
  .tab {
    display: flex;
    align-items: center;
    padding: 6px 12px;
    margin-right: 2px;
    background: #d4d4d4;
    border: 1px solid #c0c0c0;
    border-bottom: none;
    border-radius: 4px 4px 0 0;
    cursor: pointer;
    font-size: 13px;
    color: #444;
    max-width: 200px;
    min-width: 100px;
    height: 30px;
    transition: background 0.1s;
  }
  
  .tab:hover {
    background: #e6e6e6;
  }
  
  .tab.active {
    background: #fff;
    border-color: #ccc;
    color: #000;
    height: 31px; /* Overlap border */
    font-weight: 500;
    margin-bottom: -1px;
    z-index: 10;
  }
  
  .label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-right: 8px;
  }
  
  .close-btn {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
    font-size: 14px;
    line-height: 1;
  }
  
  .close-btn:hover {
    background: rgba(0,0,0,0.1);
    color: #d00;
  }
</style>
