import type { PageLoad } from './$types';
import { appState } from '$lib/api.svelte';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params }) => {
	if (appState.profiles.length === 0) {
		await appState.fetchProfiles();
	}

	const profile = appState.profiles.find((p) => p.id === params.id);
	if (!profile) {
		throw error(404, 'Profile not found');
	}

	return {
		profile
	};
};
