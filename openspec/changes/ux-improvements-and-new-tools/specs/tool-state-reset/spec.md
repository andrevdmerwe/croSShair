# Capability Spec: Tool State Reset

## Requirements

### REQ-RESET-001: Reset on Last Tab Close
- **WHEN** the last tab is closed (tabs array becomes empty)
- **THEN** the tool store MUST reset to default values:
  - Selected tool: `select`
  - Color: `#ff0000`
  - Line width: `2`

### REQ-RESET-002: Preserve State with Tabs Open
- **WHEN** tabs remain open
- **THEN** the tool state MUST persist across tab switches

## Constraints
- Reset function should be in `tools.svelte.ts`.
- Tab close detection should be in `tabs.svelte.ts`.
