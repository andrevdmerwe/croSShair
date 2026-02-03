# croSShair – Full Functional Specification

This document is the complete specification for building the **croSShair** application using an AI IDE such as Google Antigravity. It is written so an agent can implement the entire project step‑by‑step.

You **must** follow the phased development and testing process described here and wait for my approval between phases.

---

## 0. Project Overview

### 0.1 Purpose

Build **croSShair**, a lightweight, high‑performance screenshot and annotation tool for **CachyOS Linux** running the **Niri** Wayland compositor.

The application will:

- Capture **region screenshots** only, using `grim` + `slurp`.[web:11][web:32][web:17][web:37]  
- Offer a **tabbed interface** (browser‑style) where each new screenshot opens in a separate tab for editing.  
- Provide essential annotation tools: color selection, highlighting, rectangles (outline and filled), ellipses (outline and filled), lines, arrows, pixelation, step numbering markers, text, cropping, and freehand drawing.  
- Automatically save captured images to a configurable directory as PNG files named `YYYYMMDD_HHMMSS.png`.  
- Allow copying screenshots (and optionally annotated results) to the Wayland clipboard using `wl-copy` from `wl-clipboard`.[web:11]  

The emphasis is on **functionality first**; theming and visual polish will be added only after core behavior is implemented and accepted.

### 0.2 Project Identity

- Application name: **croSShair** (use this exact casing in all UI, docs, and display names).  
- Default command name / binary name: `croSShair`.  
  - If the platform requires lower‑case binaries (e.g. packaging), you may use `crosshair` as the executable name, but you must clearly document the mapping and ensure the user‑facing name remains `croSShair`.

---

## 1. Target Environment

### 1.1 Platform

- Operating system: **CachyOS Linux**, x86_64.  
- Display server: **Wayland**, compositor: **Niri**.  
- Assume wl‑roots‑style semantics for screenshot tools and clipboard utilities.

### 1.2 External CLI Dependencies

croSShair will rely on existing Wayland utilities:

- `grim` – for grabbing images from the Wayland compositor.[web:11][web:32]  
- `slurp` – for interactive region selection under Wayland.[web:17][web:37]  
- `wl-copy` and `wl-paste` – from `wl-clipboard` for clipboard interaction.[web:11]  

**Requirements:**

- On startup, croSShair must check that `grim`, `slurp`, and `wl-copy` are available in `PATH`.  
- If any are missing, croSShair must display a non‑blocking warning banner or dialog with suggested installation commands (e.g. `sudo pacman -S grim slurp wl-clipboard`).  
- The app should remain usable for non‑dependent functionality (e.g. viewing existing images) even if some dependencies are missing.

---

## 2. Technology Stack

### 2.1 Desktop Shell

- Use **Tauri** as the desktop shell (Rust backend + WebView‑based frontend), optimized for Linux/Wayland performance.[web:21][web:30]  

### 2.2 Frontend

- Use a web‑based frontend (HTML/TypeScript + UI framework).  
- The AI must select **one** of: React, Svelte, or Vue.  
- The choice must be made primarily on performance grounds (startup time, runtime memory, and canvas performance) and documented in the repository README.

### 2.3 Performance Considerations

- Enable any relevant Tauri/WebKit/WebView performance options that benefit Linux/Wayland (e.g. hardware acceleration) where stable and documented for the versions used.[web:21][web:30]  
- Keep dependencies light; avoid large CSS frameworks unless justified.  

If strong evidence emerges that a Rust‑native GUI (e.g. egui/iced) would materially improve performance, the agent may propose a stack adjustment **before Phase 3**, but must wait for explicit approval before changing direction.

---

## 3. High‑Level Architecture

### 3.1 Components

1. **Rust Backend (Tauri Core)**  
   - Responsible for:
     - Invoking `slurp` to obtain region geometry.  
     - Invoking `grim` to capture screenshots (including piping to `wl-copy` when needed).[web:11][web:32][web:17]  
     - Implementing a small IPC layer for interaction with the frontend (Tauri commands).  
     - Implementing a CLI interface for `croSShair capture-region` to integrate with Niri keybindings.

2. **Frontend UI (Web)**  
   - A single main window with:
     - Tab bar (browser‑like) where each tab represents one screenshot editing session.  
     - Canvas area for the screenshot and annotations.  
     - Toolbars for annotation tools, color selection, and basic commands (save, copy, etc.).  
     - A Settings view.

3. **Settings Subsystem**

   - Stores:
     - Screenshot save directory.  
     - Copy‑after‑capture preference.  
     - Default annotation colors and line thickness.  
   - Lives in a config file under `$XDG_CONFIG_HOME/croSShair/config.json` or `~/.config/croSShair/config.json`.

4. **Global Shortcut / CLI Integration**

   - Provides a CLI entry point:
     - `croSShair capture-region`  
   - Intended to be bound from Niri keybindings configuration.  
   - When invoked, triggers region capture, saves the screenshot, and ensures it appears as a new tab in the running UI (or starts the app if not running).

---

## 4. Functional Requirements

### 4.1 Screenshot Capture

#### 4.1.1 Capture Mode

- Only **region capture** is required (no full screen, no active window mode).

#### 4.1.2 Capture Pipeline

When the user triggers a capture (via button or CLI):

1. The backend runs `slurp` to let the user select a region on the screen.[web:17][web:37]  
   - Default behavior (no custom format) is acceptable; parse the returned `<x>,<y> <width>x<height>` string.  
2. If the user cancels (e.g. presses Escape), treat this as **capture cancelled** and do not proceed.[web:17][web:37]  
3. On a valid region, run `grim` with `-g "<geom>"` to capture the specified rectangle.[web:11][web:32][web:36]  
4. The first version should:
   - Save the screenshot to a PNG file in the configured directory using `YYYYMMDD_HHMMSS.png` naming.  
   - Optionally, also pipe the result to `wl-copy` if "copy after capture" is enabled. Examples from grim docs: `grim -g "$(slurp)" - | wl-copy`.[web:11][web:32]  
5. After `grim` finishes successfully:
   - Notify the frontend of the new file path and metadata.  
   - The frontend opens a new tab containing this screenshot for annotation.  
6. If any step fails (e.g. `grim` exit code non‑zero, missing binary, invalid path), show a clear error message in the UI and log details to console/log file.

#### 4.1.3 UI Feedback

- Show a small toast or status message for:
  - “Region captured and saved to `<full path>`.”  
  - “Capture cancelled.”  
  - “Capture failed: `<short reason>`.”

### 4.2 Tabbed Interface

#### 4.2.1 Tab Behavior

- A horizontal **tab bar** at the top of the window, similar to a browser:
  - Each tab shows a label and a close button.  
  - The active tab is visually highlighted.  
- Each captured screenshot opens in a **new tab**.  
  - Default label: `Shot 1`, `Shot 2`, etc., or a label using the timestamp (e.g. `2026-02-03 20:58:12`).  
- Closing a tab:
  - Removes that screenshot and its annotations from memory.  
  - Does **not** delete any saved original screenshot file.  

#### 4.2.2 Empty State

- When no tabs are open:
  - Show a centered message like “No screenshots yet” and a button “Take region screenshot”.  
  - The button triggers the same capture pipeline as the CLI.

### 4.3 Annotation Tools

All editing and annotation should occur in a layered canvas per tab: a base image layer with overlay primitives.

#### 4.3.1 General Canvas Behavior

- Each tab contains:
  - The base screenshot bitmap.  
  - One or more overlay objects: shapes, lines, text, step markers, pixelation blocks.  
- Provide **per‑tab** undo/redo for all annotation actions (addition, deletion, move, transform, pixelate, crop).  
- The canvas should support panning (optional) and at least basic zoom, but zoom is not mandatory in the initial implementation if it complicates performance.

#### 4.3.2 Tool Palette

Provide a tool palette that allows the user to select among:

1. **Selection / Move tool**
   - Click on an existing object to select it.  
   - Drag to move; delete with a key or a “Delete” button.  
   - Selection outline should be visible.

2. **Pen / Freehand tool**
   - Draw freehand lines with:
     - Color from current color selection.  
     - Adjustable stroke thickness (via a global stroke setting or tool setting).

3. **Highlighter tool**
   - Behaves like pen but:
     - Uses semi‑transparent strokes (e.g. alpha 0.4–0.6) to highlight underlying content.  

4. **Rectangle tool**
   - Draw rectangles by click‑drag.  
   - Two modes:
     - Outline only.  
     - Filled rectangle (fill + outline).  
   - Stroke thickness and colors configurable.

5. **Ellipse tool**
   - Similar to rectangle but elliptical shape.  
   - Outline and filled options.

6. **Line tool**
   - Straight line between drag start and end points.  
   - Stroke thickness and color configurable.

7. **Arrow tool**
   - Straight line with arrow head.  
   - Arrowhead style can be simple (triangle) with base at the end of the line.  
   - Color/thickness follow global settings.

8. **Step numbering tool**
   - When active, clicking on the canvas places a **step marker**:
     - A numbered marker with sequential numbers: 1, 2, 3, …  
     - Visual style:
       - Either a circle containing the number with a small attached arrow to the right, or a teardrop/pin‑like shape with the number centered, similar to typical step indicators.  
     - Style is **not** user‑customizable except for color (via global color selector).  
   - Each tab tracks its own step index:
     - First marker in a tab is 1, next is 2, etc.  
   - Provide a “Reset step numbers” action per tab which sets the next marker back to 1; existing markers keep their numbers.

9. **Text tool**
   - Click to place a text box.  
   - User can type arbitrary text.  
   - Properties:
     - Font size options: small / medium / large (simple preset sizes).  
     - Font color from current color selection.  
   - Text boxes can be moved after creation.

10. **Pixelation / Blur tool**
    - User drags a rectangular region.  
    - That area of the **underlying bitmap** is pixelated or blurred to obscure content.  
    - The operation should be applied permanently to the underlying screenshot layer (i.e., not just a translucent overlay).

11. **Crop tool**
    - User defines a crop rectangle around part of the screenshot.  
    - When “Apply” is pressed:
      - The underlying bitmap is trimmed to that rectangle.  
      - All overlay objects outside the crop area are either removed or clipped; objects partially in the area should be clipped reasonably.  
      - The canvas size updates to reflect the new dimensions.  

12. **Freehand drawing (if distinct from Pen)**
    - If you treat Pen and freehand as one tool, that is acceptable; otherwise a separate freehand tool is optional with the same behavior.

#### 4.3.3 Color and Stroke Controls

- A global **Color picker**:
  - Initial preset palette (e.g. red, green, blue, yellow, orange, purple).  
  - Custom color selection (e.g. via HTML color input).  
- A global **Stroke thickness** control:
  - At least 3 levels (thin, medium, thick) with actual pixel sizes.

#### 4.3.4 Undo/Redo

- Implement per‑tab undo/redo:
  - All actions that change canvas state (new shape, move, delete, pixelate, crop, text edits, step placement) must be included.  
  - Expose undo/redo via toolbar buttons and keyboard shortcuts (e.g. Ctrl+Z / Ctrl+Shift+Z or Ctrl+Y).

### 4.4 Saving and Auto‑Save

#### 4.4.1 Automatic Save on Capture

- Immediately after a screenshot capture:
  - Save the raw screenshot image to a PNG file in the configured directory.  
  - Naming pattern: `YYYYMMDD_HHMMSS.png` using the local time in 24‑hour format.  
  - Example: `20260203_205855.png`.

#### 4.4.2 Saving Annotated Images

- Provide a **“Save As”** option for the current tab:
  - Opens a file dialog with:
    - Default directory = configured screenshot directory.  
    - Default filename = base name of original file plus suffix or same name.  
  - Saves the composite image (screenshot + all overlays) as PNG.

- Provide an **“Overwrite original”** option:
  - Saves the composite image back into the original screenshot file (same path).  
  - Must show a confirmation dialog (“This will overwrite the original screenshot. Continue?”).

#### 4.4.3 Dirty State and Close Handling

- Each tab tracks whether it has unsaved changes.  
- If the user attempts to:
  - Close a dirty tab, or  
  - Exit the application with any dirty tabs,  
  - Then show a dialog:
    - Options: “Save”, “Discard”, “Cancel”.  
- You do **not** need background auto‑saving; manual save with dirty‑state prompts is sufficient.

### 4.5 Clipboard Support

- For the **current tab**, provide a “Copy to clipboard” action:
  - Renders the current image (including annotations) to PNG in memory or to a temporary file.  
  - Pipes the image into `wl-copy` with `image/png` MIME type.[web:11]  
- Settings must include a toggle:
  - “Copy captured screenshot to clipboard automatically after capture.”
  - When enabled, the capture pipeline should pipe the raw screenshot (before annotations) directly to `wl-copy`, e.g. `grim -g "$(slurp)" - | wl-copy` or equivalent.[web:11]  

---

## 5. Settings and Configuration

### 5.1 Settings Data

Store settings in a JSON config file at:

- `$XDG_CONFIG_HOME/croSShair/config.json`  
- If `$XDG_CONFIG_HOME` is not set, fall back to `~/.config/croSShair/config.json`.

Settings should include:

- `screenshotDirectory` – string path; default `~/Pictures/Screenshots`.  
- `copyAfterCapture` – boolean; default `false`.  
- `defaultColors` – object with keys for each annotation type or one global default set.  
- `defaultStrokeWidth` – numeric or enum (“thin”, “medium”, “thick”).  

### 5.2 Settings UI

Provide a **Settings** screen (dialog or tab) with:

- Directory picker for screenshot save location.  
- Checkbox: “Copy captured screenshot to clipboard automatically.”  
- Basic color configuration (can be minimal at first, but allow at least changing default primary color).  
- Default stroke thickness selector.

Also show a short **Global shortcut guidance** section:

- Explain that global shortcuts are managed by the compositor (Niri), not croSShair.  
- Display the CLI command to be bound, e.g. `croSShair capture-region`.

Example snippet for Niri’s configuration (documentation text):

```ini
binds {
    Mod+Shift+S { spawn "croSShair" "capture-region"; }
    # or
    Mod+Shift+S { spawn-sh "croSShair capture-region"; }
}
