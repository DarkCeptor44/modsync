<script lang="ts">
	import { toast } from '$lib/toast.svelte';
	import { fade } from 'svelte/transition';

	import Textarea from '$lib/components/Textarea.svelte';
	import Button from '$lib/components/Button.svelte';
	import { appState } from '$lib/api.svelte';

	let { data } = $props();

	const profile = $derived(data.profile);

	let syncing = $state(false);

	async function handleSync() {
		if (syncing) return;

		syncing = true;
		try {
			await appState.syncProfile(profile);
		} catch (err) {
			toast.show(typeof err === 'string' ? err : 'Failed to sync profile', 'error');
		} finally {
			syncing = false;
		}
	}
</script>

<div class="mb-6 flex items-center justify-between">
	<div>
		<div class="flex items-center gap-2">
			<h1 class="text-xl font-bold text-zinc-100">Sync Profiles</h1>
			{#if syncing}
				<span class="flex h-3 w-3 items-center justify-center" transition:fade={{ duration: 100 }}>
					<span class="relative flex h-2 w-2">
						<span
							class="absolute inline-flex h-2 w-2 animate-ping rounded-full bg-cyan-400 opacity-75"
						></span>
						<span class="relative inline-flex h-2 w-2 rounded-full bg-cyan-500"></span>
					</span>
				</span>
			{/if}
		</div>
		<p class="font-mono text-xs text-zinc-500">ID: {profile.id}</p>
	</div>

	<a href="/" class="text-xs text-zinc-400 transition-colors duration-100 hover:text-zinc-200"
		>&larr; Back to Profiles</a
	>
</div>

<div class="flex flex-col gap-4 rounded-xl border border-zinc-800 bg-zinc-900/60 p-6">
	<div class="grid grid-cols-1 gap-4">
		<Button onclick={handleSync} disabled={syncing}>{syncing ? 'Syncing...' : 'Sync Mods'}</Button>
		<Textarea id="sync-progress" label="Progress" class="h-70" readonly />
	</div>
</div>
