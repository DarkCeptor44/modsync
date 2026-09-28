<script lang="ts">
	import { page } from '$app/state';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { t } from '$lib/i18n/index.svelte';

	import LangPicker from './LangPicker.svelte';

	let version = $state('');

	onMount(() => {
		getVersion();
	});

	async function getVersion() {
		version = await invoke('get_version');
	}

	function navClass(path: string) {
		const isActive = page.url.pathname === path;
		return isActive ? 'bg-zinc-800 text-cyan-400 shadow-sm' : 'text-zinc-400 hover:text-zinc-200';
	}
</script>

<header class="sticky top-0 z-50 border-b border-zinc-800/80 bg-zinc-950/80 backdrop-blur-md">
	<div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-3 sm:px-6">
		<div class="flex items-center gap-3">
			<div
				class="flex h-8 w-8 items-center justify-center rounded-lg border border-cyan-500/30 bg-cyan-500/10 font-mono text-sm font-bold text-cyan-400"
			>
				MS
			</div>
			<div class="flex flex-col">
				<span class="text-sm font-semibold tracking-wide text-zinc-100">ModSync</span>
				<span class="font-mono text-[10px] text-zinc-500">v{version}</span>
			</div>
		</div>

		<div class="flex items-center gap-3">
			<nav class="flex items-center gap-1 rounded-lg border border-zinc-800 bg-zinc-900/50 p-1">
				<a
					href="/"
					class="cursor-pointer rounded-md px-3 py-1.5 text-xs font-medium transition-colors duration-150 {navClass(
						'/'
					)}"
				>
					{t('navbar.profiles')}
				</a>
				<a
					href="/settings"
					class="cursor-pointer rounded-md px-3 py-1.5 text-xs font-medium transition-colors duration-150 {navClass(
						'/settings'
					)}"
				>
					{t('settings.title')}
				</a>
			</nav>

			<LangPicker />
		</div>
	</div>
</header>
