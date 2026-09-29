/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

export interface ConfirmOptions {
	title?: string;
	message?: string;
	itemName?: string;
	confirmText?: string;
	cancelText?: string;
	variant?: 'danger' | 'warning' | 'info';
}

class ConfirmService {
	state = $state<{
		isOpen: boolean;
		options: ConfirmOptions;
		resolve: ((value: boolean) => void) | null;
	}>({
		isOpen: false,
		options: {},
		resolve: null
	});

	show(options: ConfirmOptions): Promise<boolean> {
		if (this.state.isOpen) {
			return Promise.resolve(false);
		}

		return new Promise((resolve) => {
			this.state.options = {
				title: 'Confirm Action',
				message: 'Are you sure you want to proceed?',
				confirmText: 'Confirm',
				cancelText: 'Cancel',
				variant: 'danger',
				...options
			};
			this.state.resolve = resolve;
			this.state.isOpen = true;
		});
	}

	confirm() {
		this.state.isOpen = false;
		this.state.resolve?.(true);
		this.state.resolve = null;
	}

	cancel() {
		this.state.isOpen = false;
		this.state.resolve?.(false);
		this.state.resolve = null;
	}
}

export const confirm = new ConfirmService();
