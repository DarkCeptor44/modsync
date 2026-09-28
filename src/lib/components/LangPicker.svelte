<script lang="ts">
	import { i18n, type Language } from '$lib/i18n/index.svelte';

	let isOpen = $state(false);

	const languages = i18n.availableLanguages.sort((a, b) => a.label.localeCompare(b.label));

	function selectLanguage(code: Language) {
		i18n.setLanguage(code);
		isOpen = false;
	}

	function toggleDropdown() {
		isOpen = !isOpen;
	}

	function handleBlur(event: FocusEvent) {
		const currentTarget = event.currentTarget as HTMLElement;
		if (!currentTarget.contains(event.relatedTarget as Node)) {
			isOpen = false;
		}
	}
</script>

<div class="relative inline-block text-left" onfocusout={handleBlur}>
	<button
		type="button"
		onclick={toggleDropdown}
		class="inline-flex h-9 cursor-pointer items-center gap-2 rounded-lg border px-3 text-xs font-medium transition-all duration-100 select-none focus-visible:border-cyan-500 focus-visible:outline-none active:scale-[0.98] {isOpen
			? 'border-zinc-700 bg-zinc-800/80 text-zinc-100 shadow-sm'
			: 'border-zinc-800/80 bg-zinc-900/60 text-zinc-400 hover:border-zinc-700 hover:bg-zinc-800/50 hover:text-zinc-200'}"
		aria-expanded={isOpen}
		aria-haspopup="true"
	>
		<svg
			xmlns="http://www.w3.org/2000/svg"
			fill="none"
			viewBox="0 0 24 24"
			stroke-width="1.5"
			stroke="currentColor"
			class="h-4 w-4 shrink-0 transition-colors {isOpen ? 'text-cyan-400' : 'text-zinc-400'}"
		>
			<path
				stroke-linecap="round"
				stroke-linejoin="round"
				d="M12 21a9 9 0 1 0 0-18 9 9 0 0 0 0 18Zm0 0c2.5 0 4.5-4.03 4.5-9S14.5 3 12 3m0 18c-2.5 0-4.5-4.03-4.5-9S9.5 3 12 3m-9 9h18"
			/>
		</svg>

		<span class="font-mono tracking-wider uppercase">
			{i18n.currentLanguage}
		</span>

		<svg
			class="h-3.5 w-3.5 shrink-0 text-zinc-500 transition-transform duration-100 ease-in-out {isOpen
				? 'rotate-180 text-zinc-300'
				: ''}"
			xmlns="http://www.w3.org/2000/svg"
			viewBox="0 0 20 20"
			fill="currentColor"
		>
			<path
				fill-rule="evenodd"
				d="M5.22 8.22a.75.75 0 0 1 1.06 0L10 11.94l3.72-3.72a.75.75 0 1 1 1.06 1.06l-4.25 4.25a.75.75 0 0 1-1.06 0L5.22 9.28a.75.75 0 0 1 0-1.06Z"
				clip-rule="evenodd"
			/>
		</svg>
	</button>

	{#if isOpen}
		<div
			class="absolute right-0 z-50 mt-1.5 w-36 origin-top-right rounded-lg border border-zinc-800 bg-zinc-950/95 p-1 shadow-2xl backdrop-blur-md select-none focus-visible:outline-none"
			role="menu"
		>
			{#each languages as lang}
				{@const isActive = i18n.currentLanguage === lang.code}

				<button
					type="button"
					onclick={() => selectLanguage(lang.code)}
					class="flex w-full cursor-pointer items-center justify-between rounded-md px-2.5 py-1.5 text-left text-xs font-medium transition-colors duration-100 {isActive
						? 'bg-zinc-900 text-cyan-400'
						: 'text-zinc-300 hover:bg-zinc-900/80 hover:text-zinc-100'}"
					role="menuitem"
				>
					<span>{lang.label}</span>
					{#if isActive}
						<span class="h-1.5 w-1.5 rounded-full bg-cyan-400 shadow-[0_0_8px_rgba(34,211,238,0.6)]"
						></span>
					{/if}
				</button>
			{/each}
		</div>
	{/if}
</div>
