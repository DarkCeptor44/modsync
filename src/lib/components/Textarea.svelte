<script lang="ts">
	import type { HTMLTextareaAttributes } from 'svelte/elements';

	interface Props extends HTMLTextareaAttributes {
		id: string;
		value?: string;
		label?: string;
		error?: string;
	}

	let {
		id,
		value = $bindable(''),
		label,
		error,
		class: className = '',
		...restProps
	}: Props = $props();

	const overrideHeight = $derived(/\b(h-|min-h-)/.test(String(className ?? '')));
</script>

<div class="flex w-full flex-col gap-1.5">
	{#if label}
		<label for={id} class="text-xs font-medium text-zinc-400">
			{label}
		</label>
	{/if}

	<textarea
		{...restProps}
		bind:value
		{id}
		class="w-full resize-y rounded-lg border border-zinc-800 bg-zinc-950 p-3 font-mono text-xs text-zinc-100 scheme-dark transition-colors duration-100 placeholder:text-zinc-600 focus:border-cyan-500 focus:ring-1 focus:ring-cyan-500 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50 {error
			? 'border-rose-800 focus:border-rose-500 focus:ring-rose-500'
			: ''} {overrideHeight ? '' : 'min-h-25'} {className}"></textarea>

	{#if error}
		<span class="text-[11px] text-rose-400">{error}</span>
	{/if}
</div>
