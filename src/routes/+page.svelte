<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  import { tabStore } from "$lib/stores/tabs.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import Toast from "$lib/components/Toast.svelte";
  import Canvas from "$lib/components/Canvas.svelte";
  import type { TabData, Annotation } from "$lib/types";

  async function handleCapture() {
    try {
      // Invoke Rust capture command
      const result: any = await invoke("take_screenshot");

      if (result.success && result.filePath) {
        // Create new tab
        const newTab: TabData = {
          id: crypto.randomUUID(),
          label: result.filePath.split("/").pop() || "Screenshot",
          imagePath: result.filePath,
          originalImagePath: result.filePath, // For "Overwrite original"
          annotations: [],
          isDirty: false,
          timestamp: Date.now(),
          stepCounter: 1,
        };

        tabStore.addTab(newTab);
        toastStore.add("success", "Screenshot captured successfully");
      } else if (result.cancelled) {
        toastStore.add("info", "Capture cancelled");
      } else if (result.error) {
        toastStore.add("error", `Capture failed: ${result.error}`);
      }
    } catch (e) {
      toastStore.add("error", `Error accessing capture tool: ${e}`);
    }
  }

  onMount(() => {
    // Listen for CLI-triggered captures
    const unlisten = listen("capture-region-request", () => {
      handleCapture();
    });

    return () => {
      unlisten.then((f) => f());
    };
  });
</script>

<main class="app-container">
  <div class="header">
    <TabBar />
  </div>

  <div class="content-area">
    {#if tabStore.tabs.length === 0}
      <EmptyState onCapture={handleCapture} />
    {:else}
      {#key tabStore.activeTabId}
        {#if tabStore.getTab(tabStore.activeTabId || "")}
          <Canvas tab={tabStore.getTab(tabStore.activeTabId || "")!} />
        {/if}
      {/key}
    {/if}
  </div>

  <Toast />
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      Helvetica, Arial, sans-serif;
    overflow: hidden;
    height: 100vh;
    background: #f0f0f0;
  }

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .header {
    background: #e0e0e0;
  }

  .content-area {
    flex: 1;
    position: relative;
    overflow: hidden;
    background: #333;
  }
</style>
