<!--
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { fade } from 'svelte/transition';
	import { t } from '$lib/i18n/index.svelte';

	import Button from './Button.svelte';
	import InputField from './InputField.svelte';

	interface Props {
		id: string;
		label?: string;
		newExclusion: string;
		exclusions: string[];
	}

	let { id, label, newExclusion = $bindable(''), exclusions = $bindable([]) }: Props = $props();

	function addExclusion() {
		if (!newExclusion.trim()) return;
		if (newExclusion.trim()) {
			exclusions = [...exclusions, newExclusion.trim()];
			newExclusion = '';
		}
	}

	function removeExclusion(index: number) {
		exclusions = exclusions.filter((_, i) => i !== index);
	}
</script>

<div class="flex items-end gap-2">
	<div class="flex-1">
		<InputField
			{id}
			{label}
			bind:value={newExclusion}
			placeholder={t('form.exclusionsPlaceholder')}
			autocomplete="off"
		/>
	</div>
	<Button
		variant="secondary"
		onclick={addExclusion}
		class="h-9.5 px-4"
		disabled={!newExclusion.trim()}>{t('form.add')}</Button
	>
</div>

{#if exclusions.length > 0}
	<div class="flex flex-wrap gap-2">
		{#each exclusions as item, i}
			<span
				transition:fade={{ duration: 100 }}
				class="inline-flex items-center gap-1.5 rounded-md border border-rose-800/40 bg-rose-950/30 px-2.5 py-1 font-mono text-xs text-rose-300"
			>
				<span class="select-all">
					{item}
				</span>
				<button
					type="button"
					onclick={() => removeExclusion(i)}
					class="cursor-pointer text-rose-400 transition-colors duration-100 select-none hover:text-rose-200"
					>&times;</button
				>
			</span>
		{/each}
	</div>
{/if}
