import type { Profile, ProfileInput, Settings, SyncAction, SyncOutcome } from './types';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { DEV } from './utils';

class AppState {
	profiles = $state<Profile[]>([]);
	loading = $state(false);
	outcomes = $state<SyncOutcome[]>([]);
	currentOutcome = $state<SyncOutcome | null>(null);
	settings = $state<Settings>({ jobs: 1 });

	async addProfile(profile: ProfileInput) {
		try {
			const id = await invoke<string>('add_profile', { profile });
			if (DEV) {
				console.log('added profile', profile);
			}

			this.profiles = [...this.profiles, { ...profile, id }];
		} catch (err) {
			console.error('Failed to save profile:', err);
			throw err;
		}
	}

	clearOutcomes() {
		this.outcomes = [];
		this.currentOutcome = null;
	}

	async deleteProfile(id: string) {
		try {
			await invoke('delete_profile', { id });
			if (DEV) {
				console.log('deleted profile', id);
			}

			this.profiles = this.profiles.filter((p) => p.id !== id);
		} catch (err) {
			console.error('Failed to delete profile:', err);
			throw err;
		}
	}

	async editProfile(profile: Profile): Promise<string> {
		try {
			const newId = await invoke<string>('edit_profile', { profile });
			if (DEV) {
				console.log('updated profile', profile);
			}

			const index = this.profiles.findIndex((p) => p.id === profile.id);
			if (index !== -1) {
				this.profiles[index] = {
					...profile,
					id: newId
				};
			}

			return newId;
		} catch (err) {
			console.error('Failed to update profile:', err);
			throw err;
		}
	}

	async fetchProfiles() {
		this.loading = true;
		try {
			this.profiles = await invoke('get_profiles');
		} catch (err) {
			console.error('Failed to fetch profiles:', err);
			throw err;
		} finally {
			this.loading = false;
		}
	}

	async fetchSettings() {
		try {
			this.settings = await invoke<Settings>('get_settings');
		} catch (err) {
			console.error('Failed to fetch settings:', err);
			throw err;
		}
	}

	async saveSettings(settings: Settings) {
		try {
			await invoke('save_settings', { settings });
			if (DEV) {
				console.log('saving settings:', settings);
			}

			this.settings = settings;
		} catch (err) {
			console.error('Failed to save settings:', err);
			throw err;
		}
	}

	async syncProfile(
		profile: Profile,
		dryRun: boolean,
		onProgress?: (outcome: SyncOutcome) => void
	) {
		let unlisten: UnlistenFn | undefined;

		this.clearOutcomes();

		try {
			unlisten = await listen<SyncOutcome>('sync-progress', (event) => {
				this.outcomes = [...this.outcomes, event.payload];
				this.currentOutcome = event.payload;
				onProgress?.(event.payload);
				if (DEV) {
					console.log('sync outcome:', event.payload);
				}
			});

			await invoke('sync_profile', { profile, dryRun, settings: this.settings });

			if (DEV) {
				console.log(
					'synced profile successfully',
					$state.snapshot(profile),
					'dryRun:',
					$state.snapshot(dryRun)
				);
			}
		} catch (err) {
			console.error('Failed to sync profile:', err);
			throw err;
		} finally {
			if (unlisten) {
				unlisten();
			}
			this.currentOutcome = null;
		}
	}
}

export const appState = new AppState();
