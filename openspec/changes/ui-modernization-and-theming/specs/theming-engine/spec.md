# Capability Spec: Theming Engine

## High Level Requirement
The application must support a flexible theming system allowing users to switch between visual themes (e.g., Light, Dark) with persistent preferences. All UI colors and major spacing values should be driven by CSS variables.

## Detailed Requirements

### 1. CSS Variable Infrastructure
-   **Requirement**: All colors (backgrounds, text, borders, accents) must be defined as CSS variables in a global scope (`:root`).
-   **Requirement**: Hardcoded hex/rgb values in components must be replaced with these variables.
-   **Requirement**: Semantic naming convention (e.g., `--color-bg-primary`, `--color-text-main`, `--color-accent`) should be used.

### 2. Theme Switching & Configuration
-   **Requirement**: Users must be able to switch themes via the Settings UI (Dropdown).
-   **Requirement**: Supported built-in themes:
    -   `Catppuccin Mocha` (dark)
    -   `Nord` (dark)
    -   `Osaka Jade` (dark)
    -   `Kanagawa` (dark)
    -   `Tokyo Night` (dark)
    -   `Light` (High Readability)
-   **Requirement**: Users must be able to override specific colors (e.g., Accent, Background) for the selected theme.
-   **Requirement**: Theme selection and color overrides must be persisted and restored.

### 3. Typography & Spacing
-   **Requirement**: Font sizes and base spacing units should also be tokenized (e.g., `--spacing-md`, `--font-size-lg`) to ensure consistency.
