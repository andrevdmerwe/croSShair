# Proposal: UI Modernization and Theming

## Problem
The current application interface lacks a cohesive design system, making it look dated and inconsistent. Hardcoded colors prevent theming, and the layout (toolbars, settings) does not feel "premium" or modern. The codebase contains unused CSS and ad-hoc styles that hinder maintainability.

## Solution
Implement a comprehensive **Theming Engine** and **Modernize the UI**.
-   **CSS Variables**: Replace hardcoded values with semantic CSS variables (tokens) for colors, spacing, and typography.
-   **Theme Management**: Create a `themeStore` to manage active themes and persist preferences.
-   **UI Overhaul**: Redesign the Toolbar, Settings Modal, and Tab system using the new tokens, incorporating glassmorphism and cleaner spacing.
-   **Refactoring**: Clean up `Canvas.svelte` styles and consolidate scattered CSS.

## Capabilities
-   `theming-engine`: Core infrastructure for CSS variables and theme switching.
-   `modern-ui`: Updated component designs (Toolbar, ToolPalette, Settings, Tabs) using the new engine.

## Goals
1.  Define a semantic color palette (Primary, Surface, Background, Text, Accent, etc.).
2.  Implement at least 2 themes: **Default (Dark Modern)** and **Light**.
3.  Redesign `Toolbar` and `ToolPalette` with a floating, glassmorphism aesthetic.
4.  Modernize `SettingsModal` with better typography and input styles.
5.  Refactor `Canvas.svelte` to use the new tokens.

## Non-Goals
-   Complete rewrite of the rendering engine (Canvas logic remains the same).
-   Adding new annotation tools (focus is on look & feel).
