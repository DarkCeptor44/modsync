import { generateId } from './utils';

export type ToastType = 'success' | 'error';

export interface Toast {
	id: string;
	message: string;
	type: ToastType;
}

class ToastManager {
	#toasts = $state<Toast[]>([]);

	get items() {
		return this.#toasts;
	}

	show(message: string, type: ToastType = 'success', durationMs = 4000) {
		const id = generateId();
		this.#toasts = [...this.#toasts, { id, message, type }];

		setTimeout(() => {
			this.dismiss(id);
		}, durationMs);
	}

	dismiss(id: string) {
		this.#toasts = this.#toasts.filter((t) => t.id !== id);
	}
}

export const toast = new ToastManager();
