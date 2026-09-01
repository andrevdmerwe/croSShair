<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/core";
  import { open, save } from "@tauri-apps/plugin-dialog";
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

  /** Image formats the open dialog accepts (all decodable by the webview). */
  const OPEN_IMAGE_EXTENSIONS = [
    "png",
    "jpg",
    "jpeg",
    "webp",
    "bmp",
    "gif",
  ];

  /** Create and activate a tab for an image already on disk. */
  function addImageTab(filePath: string, fallbackLabel: string) {
    const newTab: TabData = {
      id: crypto.randomUUID(),
      label: filePath.split("/").pop() || fallbackLabel,
      imagePath: filePath,
      originalImagePath: filePath, // For "Overwrite original"
      annotations: [],
      history: [[]], // Initial empty history state
      historyIndex: 0,
      isDirty: false,
      timestamp: Date.now(),
      stepCounter: 1,
    };

    tabStore.addTab(newTab);
  }

  /**
   * Destination path for an annotated save. The canvas always exports PNG, so
   * the result is always .png regardless of the source format:
   * photo.jpg -> photo_annotated.png
   */
  function annotatedPathFor(originalPath: string): string {
    const lastDot = originalPath.lastIndexOf(".");
    const lastSep = originalPath.lastIndexOf("/");
    const hasExtension = lastDot > lastSep + 1;
    const stem = hasExtension ? originalPath.substring(0, lastDot) : originalPath;
    return stem.endsWith("_annotated") ? `${stem}.png` : `${stem}_annotated.png`;
  }

  async function handleCapture() {
    try {
      // Invoke Rust capture command with optional storage path
      const result: any = await invoke("take_screenshot", {
        // savePath: settingsStore.settings.storagePath || undefined // Debugging blank screen
        savePath: undefined,
      });

      if (result.success && result.filePath) {
        addImageTab(result.filePath, "Screenshot");
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

  async function handleOpenImage() {
    try {
      const selected = await open({
        multiple: false,
        directory: false,
        filters: [{ name: "Images", extensions: OPEN_IMAGE_EXTENSIONS }],
        defaultPath: settingsStore.settings.lastOpenDirectory || undefined,
      });

      if (typeof selected !== "string") return; // Cancelled

      addImageTab(selected, "Image");

      // Remember the directory for the next open
      const separator = selected.includes("\\") ? "\\" : "/";
      const lastIndex = selected.lastIndexOf(separator);
      if (lastIndex !== -1) {
        settingsStore.updateSetting(
          "lastOpenDirectory",
          selected.substring(0, lastIndex),
        );
        settingsStore.save();
      }

      toastStore.add("success", `Opened ${selected.split("/").pop()}`);
    } catch (e) {
      console.error("Open image failed:", e);
      toastStore.add("error", `Failed to open image: ${e}`);
    }
  }

  async function handleSave() {
    if (!currentCanvas || !tabStore.activeTabId) return;
    const tab = tabStore.getTab(tabStore.activeTabId);
    if (!tab) return;

    try {
      const originalPath = tab.originalImagePath;
      if (!originalPath) {
        // Fallback to Save As if no path
        await handleSaveAs();
        return;
      }

      const annotatedPath = annotatedPathFor(originalPath);

      const base64 = await currentCanvas.exportImage();
      await invoke("save_image", {
        base64Data: base64,
        filePath: annotatedPath,
      });

      // Update tab to track the annotated path for subsequent saves
      tabStore.updateTab(tab.id, {
        imagePath: annotatedPath,
        label: annotatedPath.split("/").pop() || "Annotated Image",
      });

      toastStore.add("success", "Image saved as annotated version");
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
    console.log("[DEBUG] handleCopy called", {
      currentCanvas: !!currentCanvas,
      activeTabId: tabStore.activeTabId,
    });
    if (!currentCanvas || !tabStore.activeTabId) {
      console.log("[DEBUG] handleCopy early return - no canvas or tab");
      return;
    }

    try {
      console.log("[DEBUG] Exporting image...");
      const base64 = await currentCanvas.exportImage();
      console.log("[DEBUG] Image exported, base64 length:", base64.length);
      console.log("[DEBUG] Calling copy_image_to_clipboard...");
      await invoke("copy_image_to_clipboard", { base64Data: base64 });
      console.log("[DEBUG] Copy successful!");
      toastStore.add("success", "Image copied to clipboard");
    } catch (e) {
      console.error("Copy failed:", e);
      toastStore.add("error", `Failed to copy to clipboard: ${e}`);
    }
  }

  // Save a specific tab by ID (used by TabBar for unsaved changes prompts)
  async function handleSaveTab(tabId: string) {
    const tab = tabStore.getTab(tabId);
    if (!tab) return;

    try {
      const originalPath = tab.originalImagePath;
      if (!originalPath) {
        // Can't auto-save without a path
        toastStore.add("info", "Cannot auto-save: no file path available");
        return;
      }

      const annotatedPath = annotatedPathFor(originalPath);

      // We need to temporarily switch to this tab to export its canvas
      const previousActiveTabId = tabStore.activeTabId;
      tabStore.activeTabId = tabId;

      // Wait a tick for the canvas to mount
      await new Promise((resolve) => setTimeout(resolve, 50));

      if (currentCanvas) {
        const base64 = await currentCanvas.exportImage();
        await invoke("save_image", {
          base64Data: base64,
          filePath: annotatedPath,
        });

        tabStore.updateTab(tabId, {
          imagePath: annotatedPath,
          label: annotatedPath.split("/").pop() || "Annotated Image",
          isDirty: false,
        });

        toastStore.add("success", `Saved: ${annotatedPath.split("/").pop()}`);
      }

      // Restore previous active tab if it still exists
      if (previousActiveTabId && tabStore.getTab(previousActiveTabId)) {
        tabStore.activeTabId = previousActiveTabId;
      }
    } catch (e) {
      console.error("Save tab failed:", e);
      toastStore.add("error", `Failed to save: ${e}`);
    }
  }

  onMount(() => {
    // Listen for CLI-triggered captures
    const unlisten = listen("capture-region-request", () => {
      handleCapture();
    });

    // Check for startup intent (cold start args)
    invoke<string | null>("check_startup_intent")
      .then((intent) => {
        if (intent === "capture-region") {
          handleCapture();
        }
      })
      .catch((e) => console.error("Failed to check startup intent:", e));

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
    <TabBar onSaveTab={handleSaveTab} />
  </div>

  <div class="content-area">
    <div class="sidebar">
      <ToolPalette
        onCapture={handleCapture}
        onOpenImage={handleOpenImage}
        onSave={handleSave}
        onSaveAs={handleSaveAs}
        onCopy={handleCopy}
      />
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
  /* Global body styles moved to src/theme.css */

  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
  }

  .header {
    background: var(--bg-secondary);
    -webkit-app-region: drag; /* Electron/Tauri standard for drag */
    /* data-tauri-drag-region on inner element for tauri v2 usually */
  }

  .content-area {
    flex: 1;
    position: relative;
    overflow: hidden;
    background: var(--bg-primary);
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
    /* The tool palette floats (position: fixed) over this area, so reserve its
       width here. Everything inside - including the zoomed canvas and its
       scroll extent - stays clear of it. */
    padding-left: var(--palette-gutter);
    box-sizing: border-box;
  }
</style>
