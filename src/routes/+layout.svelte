<!--
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';
	import { DEV } from '$lib/utils';

	import ToastProvider from '$lib/components/ToastProvider.svelte';
	import ConfirmModal from '$lib/components/ConfirmModal.svelte';
	import NavBar from '$lib/components/NavBar.svelte';

	onMount(() => {
		if (DEV) {
			console.log('in dev mode');
		}
	});

	let { children } = $props();
</script>

<ConfirmModal />

<ToastProvider />

<NavBar />

<main class="mx-auto flex w-full max-w-7xl flex-1 flex-col justify-start px-4 py-6 sm:px-6">
	{@render children()}
</main>
