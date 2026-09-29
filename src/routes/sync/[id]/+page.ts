/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

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
