export interface ToastMessage {
    id: string;
    type: 'success' | 'error' | 'info';
    message: string;
}

class ToastStore {
    toasts = $state<ToastMessage[]>([]);

    add(type: 'success' | 'error' | 'info', message: string, duration = 3000) {
        const id = crypto.randomUUID();
        this.toasts.push({ id, type, message });

        setTimeout(() => {
            this.remove(id);
        }, duration);
    }

    remove(id: string) {
        this.toasts = this.toasts.filter(t => t.id !== id);
    }
}

export const toastStore = new ToastStore();
