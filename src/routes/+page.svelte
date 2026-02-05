<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { save } from "@tauri-apps/plugin-dialog";
  import { onMount } from "svelte";

  import { tabStore } from "$lib/stores/tabs.svelte";
  import { settingsStore } from "$lib/stores/settings.svelte";
  import { toastStore } from "$lib/stores/toast.svelte";
  import TabBar from "$lib/components/TabBar.svelte";
  import EmptyState from "$lib/components/EmptyState.svelte";
  import Toast from "$lib/components/Toast.svelte";
  import ToolPalette from "$lib/components/ToolPalette.svelte";
  import Canvas from "$lib/components/Canvas.svelte";
  import type { TabData, Annotation } from "$lib/types";

  let currentCanvas: Canvas | undefined = $state();

  async function handleCapture() {
    try {
      // Invoke Rust capture command with optional storage path
      const result: any = await invoke("take_screenshot", {
        // savePath: settingsStore.settings.storagePath || undefined // Debugging blank screen
        savePath: undefined,
      });

      if (result.success && result.filePath) {
        // Create new tab
        const newTab: TabData = {
          id: crypto.randomUUID(),
          label: result.filePath.split("/").pop() || "Screenshot",
          imagePath: result.filePath,
          originalImagePath: result.filePath, // For "Overwrite original"
          annotations: [],
          history: [[]], // Initial empty history state
          historyIndex: 0,
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

  async function handleSave() {
    if (!currentCanvas || !tabStore.activeTabId) return;
    const tab = tabStore.getTab(tabStore.activeTabId);
    if (!tab) return;

    try {
      // Direct overwrite if we have a path
      const filePath = tab.originalImagePath;
      if (!filePath) {
        // Fallback to Save As if no path
        await handleSaveAs();
        return;
      }

      const base64 = await currentCanvas.exportImage();
      await invoke("save_image", { base64Data: base64, filePath });
      toastStore.add("success", "Image saved");
    } catch (e) {
      console.error("Save failed:", e);
      toastStore.add("error", `Failed to save: ${e}`);
    }
  }

  async function handleSaveAs() {
    if (!currentCanvas || !tabStore.activeTabId) return;
    const tab = tabStore.getTab(tabStore.activeTabId);
    if (!tab) return;

    try {
      const base64 = await currentCanvas.exportImage();

      const suggestedName = tab.label.endsWith(".png")
        ? tab.label
        : `${tab.label}.png`;
      let defaultPath = suggestedName;

      // Prepend last directory if available
      if (settingsStore.settings.lastSaveDirectory) {
        const dir = settingsStore.settings.lastSaveDirectory;
        // Ensure proper separator (simple check)
        const sep = dir.includes("\\") ? "\\" : "/";
        const cleanDir = dir.endsWith(sep) ? dir : `${dir}${sep}`;
        defaultPath = `${cleanDir}${suggestedName}`;
      }

      const filePath = await save({
        filters: [
          {
            name: "Image",
            extensions: ["png"],
          },
        ],
        defaultPath,
      });

      if (filePath) {
        await invoke("save_image", { base64Data: base64, filePath });

        // Update tab info to reflect new location
        tabStore.updateTab(tab.id, {
          imagePath: filePath,
          originalImagePath: filePath,
          label: filePath.split("/").pop() || "Saved Image",
        });

        // Persist directory
        const separator = filePath.includes("\\") ? "\\" : "/";
        const lastIndex = filePath.lastIndexOf(separator);
        if (lastIndex !== -1) {
          const dir = filePath.substring(0, lastIndex);
          settingsStore.updateSetting("lastSaveDirectory", dir);
          settingsStore.save(); // Ensure it saves to disk
        }

        toastStore.add("success", "Image saved successfully");
      }
    } catch (e) {
      console.error("Save As failed:", e);
      toastStore.add("error", `Failed to save image: ${e}`);
    }
  }

  async function handleCopy() {
    if (!currentCanvas || !tabStore.activeTabId) return;

    try {
      const base64 = await currentCanvas.exportImage();
      await invoke("copy_image_to_clipboard", { base64Data: base64 });
      toastStore.add("success", "Image copied to clipboard");
    } catch (e) {
      console.error("Copy failed:", e);
      toastStore.add("error", `Failed to copy to clipboard: ${e}`);
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

  // Auto-save settings when they change
  $effect(() => {
    settingsStore.save();
  });
</script>

<main class="app-container">
  <div class="header">
    <TabBar />
  </div>

  <div class="content-area">
    <div class="sidebar">
      <div class="sidebar">
        <ToolPalette
          onCapture={handleCapture}
          onSave={handleSave}
          onSaveAs={handleSaveAs}
          onCopy={handleCopy}
        />
      </div>
    </div>
    <div class="canvas-viewport">
      {#if tabStore.tabs.length === 0}
        <EmptyState onCapture={handleCapture} />
      {:else}
        {#key tabStore.activeTabId}
          {#if tabStore.getTab(tabStore.activeTabId || "")}
            <Canvas
              bind:this={currentCanvas}
              tab={tabStore.getTab(tabStore.activeTabId || "")!}
            />
          {/if}
        {/key}
      {/if}
    </div>
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
    display: flex;
    flex-direction: row;
  }

  .sidebar {
    flex: 0 0 auto;
    height: 100%;
    z-index: 10;
  }

  .canvas-viewport {
    flex: 1;
    position: relative;
    overflow: hidden;
    display: flex; /* Ensure Canvas component fills space */
  }
</style>
