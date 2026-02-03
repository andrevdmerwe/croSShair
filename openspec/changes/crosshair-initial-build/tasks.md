## 1. Project Setup

- [ ] 1.1 Initialize Tauri project with Svelte frontend using `npm create tauri-app`
- [ ] 1.2 Configure Tauri for single-instance behavior using `tauri-plugin-single-instance`
- [ ] 1.3 Set up project structure (src-tauri for Rust, src for Svelte frontend)
- [ ] 1.4 Add necessary dependencies to Cargo.toml (serde, directories, chrono)
- [ ] 1.5 Configure Tauri for Wayland/WebKitGTK with GPU acceleration

## 2. Configuration Subsystem

- [ ] 2.1 Implement XDG-compliant config path resolution in Rust backend
- [ ] 2.2 Create config.json schema with screenshotDirectory, copyAfterCapture, defaultColors, defaultStrokeWidth
- [ ] 2.3 Implement config load/save Tauri commands
- [ ] 2.4 Create default config initialization with ~/Pictures/Screenshots default directory

## 3. CLI Tool Integration

- [ ] 3.1 Implement Rust function to check PATH for grim, slurp, wl-copy
- [ ] 3.2 Create dependency status Tauri command for frontend
- [ ] 3.3 Implement slurp invocation to get region geometry
- [ ] 3.4 Implement grim invocation with geometry parameter and output path
- [ ] 3.5 Implement wl-copy piping for clipboard integration
- [ ] 3.6 Add timestamp-based filename generation (YYYYMMDD_HHMMSS.png)

## 4. CLI Entry Point

- [ ] 4.1 Add CLI argument parsing for `capture-region` subcommand
- [ ] 4.2 Implement single-instance IPC to send capture command to running instance
- [ ] 4.3 Handle app startup when no instance is running
- [ ] 4.4 Create Tauri event for new screenshot from CLI

## 5. Frontend Shell

- [ ] 5.1 Create main App.svelte layout with tab bar, toolbar, and canvas area
- [ ] 5.2 Implement Tab store with add/remove/select functionality
- [ ] 5.3 Create TabBar component with label, close button, active highlighting
- [ ] 5.4 Implement empty state view with "No screenshots yet" message and capture button
- [ ] 5.5 Add toast notification system for capture feedback

## 6. Canvas System

- [ ] 6.1 Create Canvas component with HTML5 Canvas element
- [ ] 6.2 Implement base layer for screenshot bitmap rendering
- [ ] 6.3 Implement overlay layer for annotation objects
- [ ] 6.4 Create annotation object model (type, position, color, stroke, data)
- [ ] 6.5 Implement canvas resize handling when tab changes

## 7. Selection and Transform

- [ ] 7.1 Implement hit-testing for annotation objects
- [ ] 7.2 Create selection state with visible outline rendering
- [ ] 7.3 Implement drag-to-move for selected objects
- [ ] 7.4 Add Delete key and button handling for selected objects

## 8. Drawing Tools

- [ ] 8.1 Implement tool state machine (current tool, tool options)
- [ ] 8.2 Create Pen tool with freehand path recording
- [ ] 8.3 Create Highlighter tool with semi-transparent stroke (alpha 0.4-0.6)
- [ ] 8.4 Create Rectangle tool with outline/filled mode toggle
- [ ] 8.5 Create Ellipse tool with outline/filled mode toggle
- [ ] 8.6 Create Line tool for straight lines
- [ ] 8.7 Create Arrow tool with triangle arrowhead rendering

## 9. Special Tools

- [ ] 9.1 Create Step numbering tool with sequential counter per tab
- [ ] 9.2 Implement step marker rendering (circle/teardrop with number)
- [ ] 9.3 Add "Reset step numbers" action per tab
- [ ] 9.4 Create Text tool with click-to-place text box
- [ ] 9.5 Implement text editing with font size presets (small/medium/large)

## 10. Destructive Tools

- [ ] 10.1 Implement Pixelation tool with region selection
- [ ] 10.2 Create pixelation algorithm to apply to base layer bitmap
- [ ] 10.3 Implement Crop tool with rectangle definition
- [ ] 10.4 Create crop apply logic to trim bitmap and clip/remove annotations

## 11. Tool Controls

- [ ] 11.1 Create ToolPalette component with all tool buttons
- [ ] 11.2 Implement ColorPicker with preset palette (6 colors) and custom input
- [ ] 11.3 Create StrokeWidth selector with thin/medium/thick options
- [ ] 11.4 Wire tool controls to annotation creation

## 12. History System

- [ ] 12.1 Implement per-tab undo/redo stack
- [ ] 12.2 Create undoable action wrapper for all canvas mutations
- [ ] 12.3 Add Ctrl+Z / Ctrl+Shift+Z keyboard shortcuts
- [ ] 12.4 Create Undo/Redo toolbar buttons

## 13. Save and Export

- [ ] 13.1 Implement composite image rendering (base + annotations) to PNG
- [ ] 13.2 Create "Save As" dialog with file picker
- [ ] 13.3 Implement "Overwrite original" with confirmation dialog
- [ ] 13.4 Implement "Copy to clipboard" action via wl-copy
- [ ] 13.5 Track dirty state per tab for unsaved changes

## 14. Dirty State Handling

- [ ] 14.1 Add dirty flag to tab model, update on annotation changes
- [ ] 14.2 Implement close confirmation dialog (Save/Discard/Cancel)
- [ ] 14.3 Implement exit confirmation when any tab is dirty

## 15. Settings UI

- [ ] 15.1 Create Settings component/view
- [ ] 15.2 Add directory picker for screenshot save location
- [ ] 15.3 Add checkbox for "Copy to clipboard automatically"
- [ ] 15.4 Add default color selector
- [ ] 15.5 Add default stroke thickness selector
- [ ] 15.6 Add global shortcut guidance section with Niri example

## 16. Final Integration

- [ ] 16.1 Wire capture pipeline: CLI → backend → new tab in frontend
- [ ] 16.2 Test single-instance behavior (CLI capture while app running)
- [ ] 16.3 Test all annotation tools end-to-end
- [ ] 16.4 Test save/export flows
- [ ] 16.5 Test settings persistence
