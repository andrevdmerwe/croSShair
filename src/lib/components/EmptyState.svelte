<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    let { onCapture } = $props<{ onCapture: () => void }>();
</script>

<div class="empty-state">
    <div class="content">
        <div class="icon">✜</div>
        <h2>No screenshots yet</h2>
        <p>Capture a region to start editing</p>

        <button class="capture-btn" onclick={onCapture}>
            Take region screenshot
        </button>

        <p class="hint">
            Or run <code class="code">croSShair capture-region</code>
        </p>
    </div>
</div>

<style>
    .empty-state {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        height: 100%;
        width: 100%;
        background: var(--bg-primary);
        color: var(--text-muted);
    }

    .content {
        text-align: center;
        max-width: 300px;
    }

    .icon {
        font-size: 48px;
        margin-bottom: 16px;
        color: var(--text-muted);
        opacity: 0.5;
    }

    h2 {
        margin: 0 0 8px 0;
        color: var(--text-main);
        font-weight: 500;
    }

    p {
        margin: 0 0 24px 0;
    }

    .capture-btn {
        background: var(--accent);
        color: var(--bg-primary); /* Contrast text on accent */
        border: none;
        padding: 10px 20px;
        border-radius: var(--radius-sm);
        font-size: 15px;
        font-weight: 600;
        cursor: pointer;
        box-shadow: var(--shadow-sm);
        transition:
            opacity 0.2s,
            transform 0.1s;
    }

    .capture-btn:hover {
        opacity: 0.9;
        transform: translateY(-1px);
    }

    .hint {
        margin-top: 32px;
        font-size: 13px;
        opacity: 0.8;
    }

    .code {
        background: var(--bg-secondary);
        padding: 2px 6px;
        border-radius: 3px;
        font-family: monospace;
        color: var(--text-main);
    }
</style>
