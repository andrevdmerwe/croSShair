# Tasks

- [x] **State**: Create `SettingsStore` <!-- id: 1 -->
    - [ ] Define default settings constant
    - [ ] Create `SettingsStore` using Svelte 5 `$state`
    - [ ] Implement `localStorage` sync in `$effect`
    - [ ] Export store for global use

- [x] **UI**: Create Settings Component <!-- id: 2 -->
    - [x] Create `SettingsModal.svelte` accessible via a gear icon
    - [x] Implement UI for "Defaults" section (Tool dropdown, Width slider, Density slider)
    - [x] Implement UI for "Palette" section (Color pickers for the 8 slots)
    - [x] Implement UI for "Storage" section (Path display + "Browse" button)

- [x] **Integration**: Connect Toolbar & Canvas to Settings <!-- id: 3 -->
    - [x] Update `ToolPalette.svelte` to render colors from `settingsStore.palette` instead of hardcoded list
    - [x] Update `ToolPalette.svelte` to initialize active tool/width/color from `settingsStore` on mount
    - [x] Update `Canvas.svelte` / `toolStore` logic to respect defaults

- [x] **Backend**: Storage Path Selection <!-- id: 4 -->
    - [x] Wire up "Browse" button in Settings to Tauri's `open` dialog (directory selection)
    - [x] Update screenshot saving logic to use the path from settings

- [x] **Verification**: Settings Persistence <!-- id: 5 -->
    - [x] Verify settings persist after reload
    - [x] Verify custom palette colors apply correctly
