<script lang="ts">
	import { appState } from '$lib/api.svelte';
	import { onMount } from 'svelte';
	import { DEV } from '$lib/utils';
	import { t } from '$lib/i18n/index.svelte';

	import Button from '$lib/components/Button.svelte';

	onMount(() => {
		appState.fetchProfiles();

		if (DEV) {
			console.log('profiles', $state.snapshot(appState.profiles));
		}
	});
</script>

<div class="mb-6 flex items-center justify-between">
	<div>
		<h1 class="text-xl font-bold text-zinc-100">{t('home.title')}</h1>
		<p class="text-xs text-zinc-400">{t('home.subtitle')}</p>
	</div>

	<Button href="/profiles" textSize="text-sm">{t('home.addButton')}</Button>
</div>

{#if appState.loading && appState.profiles.length === 0}
	<div class="flex flex-col gap-3">
		{#each Array(2) as _}
			<div class="h-24 animate-pulse rounded-xl border border-zinc-800 bg-zinc-900/50 p-4"></div>
		{/each}
	</div>
{:else if appState.profiles.length === 0}
	<div class="rounded-xl border border-dashed border-zinc-800 bg-zinc-900/30 p-8 text-center">
		<p class="text-xs text-zinc-400">{t('home.empty')}</p>
	</div>
{:else}
	<div class="flex flex-col gap-3">
		{#each appState.profiles as profile (profile.id)}
			{@const exclusions = profile.syncExclusions.length + profile.deleteExclusions.length}

			<div
				class="flex items-center justify-between rounded-xl border border-zinc-800 bg-zinc-900/60 p-4"
			>
				<div class="flex min-w-0 flex-col gap-1 pr-4">
					<div class="flex items-center gap-2">
						<span class="truncate font-semibold text-zinc-100">{profile.name}</span>
						{#if exclusions > 0}
							<span
								class="rounded border border-zinc-800 bg-zinc-950 px-1.5 py-0.5 text-[10px] text-zinc-400"
							>
								{t('home.exclusionCount', { count: exclusions.toString() })}
							</span>
						{/if}
					</div>
					<div class="flex items-center gap-2 font-mono text-[11px] text-zinc-400">
						<span class="truncate" title={profile.source}>{profile.source}</span>
						<span>→</span>
						<span class="truncate" title={profile.destination}>{profile.destination}</span>
					</div>
				</div>

				<div class="flex shrink-0 items-center gap-2">
					<Button href="/profiles/{profile.id}" textSize="text-xs">{t('home.edit')}</Button>
					<Button href="/sync/{profile.id}" textSize="text-xs">{t('home.sync')}</Button>
				</div>
			</div>
		{/each}
	</div>
{/if}
