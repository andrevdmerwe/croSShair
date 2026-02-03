# croSShair

Lightweight, high-performance screenshot and annotation tool for **CachyOS Linux** with **Niri** Wayland compositor.

## Features

- 📸 Region screenshot capture using `grim` + `slurp`
- 🏷️ Browser-style tabbed interface for multiple screenshots
- ✏️ Annotation tools: freehand, highlighter, shapes, arrows, text, step markers
- 🔢 Step numbering for tutorials and guides
- 🔳 Pixelation/blur for sensitive information
- ✂️ Crop and resize
- 📋 Wayland clipboard integration via `wl-copy`
- 💾 Auto-save with timestamp naming (`YYYYMMDD_HHMMSS.png`)

## Why Svelte?

This project uses **Svelte** as the frontend framework for the following performance reasons:

| Framework | Bundle Size | Virtual DOM | Runtime Overhead |
|-----------|-------------|-------------|------------------|
| **Svelte** | ~2-5KB | ❌ None | Compile-time reactivity |
| React | ~40KB | ✅ Yes | Runtime diffing |
| Vue | ~30KB | ✅ Yes | Runtime reactivity |

For a performance-focused, lightweight desktop app with canvas-heavy interactions, Svelte provides:
- **Faster startup** – smaller bundle = quicker load
- **Better canvas performance** – no VDOM reconciliation overhead
- **Lower memory footprint** – critical for annotation editing

## Requirements

### System Dependencies

```bash
# CachyOS / Arch Linux
sudo pacman -S grim slurp wl-clipboard
```

| Tool | Purpose |
|------|---------|
| `grim` | Wayland screenshot capture |
| `slurp` | Region selection |
| `wl-clipboard` | Clipboard integration (wl-copy, wl-paste) |

## Development

```bash
# Install dependencies
npm install

# Start development server
npm run tauri dev

# Build for production
npm run tauri build
```

## Niri Keybinding

Add to your Niri configuration:

```kdl
binds {
    Mod+Shift+S { spawn "croSShair" "capture-region"; }
}
```

## Tech Stack

- **Backend**: Rust + Tauri 2.0
- **Frontend**: Svelte + TypeScript + Vite
- **Target**: Linux x86_64 / Wayland / Niri

## License

MIT
