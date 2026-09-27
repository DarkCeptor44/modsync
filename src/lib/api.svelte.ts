import type { Profile, ProfileInput, SyncAction, SyncOutcome } from './types';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { DEV } from './utils';

class AppState {
	profiles = $state<Profile[]>([]);
	loading = $state(false);
	outcomes = $state<SyncOutcome[]>([]);
	currentOutcome = $state<SyncOutcome | null>(null);

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

	async syncProfile(profile: Profile, onProgress?: (outcome: SyncOutcome) => void) {
		let unlisten: UnlistenFn | undefined;

		try {
			unlisten = await listen<SyncOutcome>('sync-progress', (event) => {
				this.outcomes = [...this.outcomes, event.payload];
				this.currentOutcome = event.payload;
				onProgress?.(event.payload);
				if (DEV) {
					console.log('sync outcome:', event.payload);
				}
			});

			await invoke('sync_profile', { profile });

			if (DEV) {
				console.log('synced profile successfully', $state.snapshot(profile));
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
