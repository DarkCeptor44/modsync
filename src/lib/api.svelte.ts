import type { Profile, ProfileInput } from './types/profile';
import { invoke } from '@tauri-apps/api/core';
import { DEV } from './utils';

class AppState {
	profiles = $state<Profile[]>([]);
	loading = $state(false);

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

	async syncProfile(profile: Profile) {
		try {
			await invoke('sync_profile', { profile });
			if (DEV) {
				console.log('syncing profile', profile);
			}
		} catch (err) {
			console.error('Failed to sync profile:', err);
			throw err;
		}
	}
}

export const appState = new AppState();
