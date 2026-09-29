<!--
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import type { Log } from '$lib/components/LogBox.svelte';
	import { DEV } from '$lib/utils.js';
	import { appState } from '$lib/api.svelte';
	import { onMount } from 'svelte';
	import { toast } from '$lib/toast.svelte';
	import { fade } from 'svelte/transition';
	import { t } from '$lib/i18n/index.svelte';

	import Button from '$lib/components/Button.svelte';
	import Toggle from '$lib/components/Toggle.svelte';
	import LogBox from '$lib/components/LogBox.svelte';

	let { data } = $props();
	const profile = $derived(data.profile);

	onMount(() => {
		if (DEV) {
			console.log('profile:', $state.snapshot(profile));
		}
	});

	let syncing = $state(false);
	let dryRun = $state(false);
	let showSkipped = $state(false);
	let logs = $derived<Log[]>([]);

	async function handleSync() {
		if (syncing) return;

		logs = [];
		appState.clearOutcomes();
		syncing = true;

		logs = [
			...logs,
			{
				kind: 'system',
				level: 'info',
				message: `Starting sync for profile ${profile.name}`
			}
		];

		try {
			await appState.syncProfile(profile, dryRun, (out) => {
				logs = [...logs, { kind: 'outcome', outcome: out }];
			});

			logs = [
				...logs,
				{ kind: 'system', level: 'success', message: 'Sync completed successfully.' }
			];
			toast.show(t('sync.synced'));
		} catch (err) {
			const errorMsg = typeof err === 'string' ? err : t('sync.failed');
			logs = [...logs, { kind: 'system', level: 'error', message: errorMsg }];
			toast.show(errorMsg, 'error');
		} finally {
			syncing = false;
		}
	}
</script>

<div class="mb-6 flex items-center justify-between">
	<div>
		<div class="flex items-center gap-2">
			<h1 class="text-xl font-bold text-zinc-100">{t('sync.title')}</h1>
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
		>&larr; {t('common.back')}</a
	>
</div>

<div class="flex flex-col gap-4 rounded-xl border border-zinc-800 bg-zinc-900/60 p-6">
	<div class="grid grid-cols-1 gap-4">
		<div class="flex items-center gap-3">
			<Toggle
				id="sync-dry-run"
				disabled={syncing}
				bind:checked={dryRun}
				checkedColorClass="bg-emerald-500"
				uncheckedColorClass="bg-amber-500">{t('sync.dryRun')}</Toggle
			>
			<Toggle id="sync-show-skip" bind:checked={showSkipped}>{t('sync.showSkipped')}</Toggle>
			<Button onclick={handleSync} disabled={syncing} class="flex-1" textSize="text-sm">
				{syncing
					? dryRun
						? t('sync.simulating')
						: t('sync.syncing')
					: dryRun
						? t('sync.simulate')
						: t('sync.sync')}
			</Button>
		</div>
		<LogBox id="sync-progress" {logs} label={t('sync.logs')} {dryRun} {showSkipped} />
	</div>
</div>
