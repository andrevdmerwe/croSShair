<script lang="ts">
    import { toastStore } from "../stores/toast.svelte";
    import { fade, fly } from "svelte/transition";
</script>

<div class="toast-container">
    {#each toastStore.toasts as toast (toast.id)}
        <div
            class="toast {toast.type}"
            in:fly={{ y: 20, duration: 300 }}
            out:fade={{ duration: 200 }}
        >
            {toast.message}
        </div>
    {/each}
</div>

<style>
    .toast-container {
        position: fixed;
        bottom: 24px;
        right: 24px;
        z-index: 9999;
        display: flex;
        flex-direction: column;
        gap: 10px;
        pointer-events: none;
    }

    .toast {
        background: #333;
        color: white;
        padding: 12px 20px;
        border-radius: 4px;
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        font-size: 14px;
        min-width: 250px;
        pointer-events: auto;
        display: flex;
        align-items: center;
    }

    .toast.success {
        background: #2e7d32;
    }

    .toast.error {
        background: #d32f2f;
    }

    .toast.info {
        background: #0288d1;
    }
</style>
