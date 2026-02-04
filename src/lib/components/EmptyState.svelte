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
        background: #f5f5f5;
        color: #666;
    }

    .content {
        text-align: center;
        max-width: 300px;
    }

    .icon {
        font-size: 48px;
        margin-bottom: 16px;
        color: #888;
    }

    h2 {
        margin: 0 0 8px 0;
        color: #333;
        font-weight: 500;
    }

    p {
        margin: 0 0 24px 0;
    }

    .capture-btn {
        background: #007bff;
        color: white;
        border: none;
        padding: 10px 20px;
        border-radius: 4px;
        font-size: 15px;
        font-weight: 500;
        cursor: pointer;
        box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        transition: background 0.2s;
    }

    .capture-btn:hover {
        background: #0069d9;
    }

    .hint {
        margin-top: 32px;
        font-size: 13px;
        opacity: 0.8;
    }

    .code {
        background: #e0e0e0;
        padding: 2px 6px;
        border-radius: 3px;
        font-family: monospace;
    }
</style>
