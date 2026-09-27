<script lang="ts">
	import type { SyncOutcome } from '$lib/types.js';
	import { appState } from '$lib/api.svelte';
	import { onMount } from 'svelte';
	import { toast } from '$lib/toast.svelte';
	import { fade } from 'svelte/transition';
	import { DEV } from '$lib/utils.js';

	import Textarea from '$lib/components/Textarea.svelte';
	import Button from '$lib/components/Button.svelte';
	import Toggle from '$lib/components/Toggle.svelte';

	let { data } = $props();

	const profile = $derived(data.profile);

	onMount(() => {
		if (DEV) {
			console.log('profile:', $state.snapshot(profile));
		}
	});

	let syncing = $state(false);
	let dryRun = $state(false);
	let logOutput = $state('');

	async function handleSync() {
		if (syncing) return;

		logOutput = '';
		syncing = true;
		appendLog(`[INFO] Starting sync for profile: ${profile.name}`);

		try {
			await appState.syncProfile(profile, dryRun, (out) => {
				const formatted = formatOutcome(out);
				if (formatted) {
					appendLog(formatted);
				}
			});
			appendLog('[SUCCESS] Sync completed.');
			toast.show('Profile synced successfully');
		} catch (err) {
			const errorMsg = typeof err === 'string' ? err : 'Failed to sync profile';
			appendLog(`[ERROR] ${errorMsg}`);
			toast.show(errorMsg, 'error');
		} finally {
			syncing = false;
		}
	}

	function appendLog(message: string) {
		logOutput = logOutput
			? `${logOutput}\n${dryRun ? '[DRY-RUN] ' : ''}${message}`
			: `${dryRun ? '[DRY-RUN] ' : ''}${message}`;
	}

	function formatOutcome(out: SyncOutcome): string {
		const action = out.action;
		if (typeof action === 'string') {
			if (action === 'RemovedDir') return `- ${out.entry}`;
			return '';
		}
		if ('Copied' in action) return `+ ${out.entry} (${action.Copied.bytes} bytes)`;
		if ('Removed' in action) return `- ${out.entry} (${action.Removed.bytes} bytes)`;
		if ('Failed' in action) return `x ${out.entry}: ${action.Failed.error}`;
		return '';
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
		<div class="flex items-center gap-3">
			<Toggle
				id="dry-run-sync"
				disabled={syncing}
				bind:checked={dryRun}
				checkedColorClass="bg-emerald-500"
				uncheckedColorClass="bg-amber-500">Dry Run</Toggle
			>
			<Button onclick={handleSync} disabled={syncing} class="flex-1" textSize="text-sm">
				{syncing ? (dryRun ? 'Simulating...' : 'Syncing...') : dryRun ? 'Simulate' : 'Sync Mods'}
			</Button>
		</div>
		<Textarea
			bind:value={logOutput}
			id="sync-progress"
			label="Progress"
			class="h-70"
			readonly
			placeholder="Sync output will appear here..."
		/>
	</div>
</div>
