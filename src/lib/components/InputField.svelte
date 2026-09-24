<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	interface Props extends HTMLInputAttributes {
		label?: string;
		id: string;
		type?: 'text' | 'number' | 'time' | 'date' | 'url' | 'password';
		placeholder?: string;
		value: any;
		required?: boolean;
		transform?: (value: string) => string;
		element?: HTMLInputElement | null;
	}

	let {
		label,
		id,
		type = 'text',
		placeholder = '',
		value = $bindable(),
		required = false,
		transform,
		element = $bindable(null),
		class: className = '',
		oninput,
		...restProps
	}: Props = $props();

	function handleInput(e: Event & { currentTarget: HTMLInputElement }) {
		let newValue = e.currentTarget.value;

		if (transform) {
			newValue = transform(newValue);
			e.currentTarget.value = newValue;
		}

		value = newValue;

		if (typeof oninput === 'function') {
			oninput(e);
		}
	}
</script>

<div class="flex flex-col gap-1.5">
	<label for={id} class="text-xs font-medium text-zinc-400">
		<span>{label}</span>
		{#if required}
			<span class="text-rose-400">*</span>
		{/if}
	</label>

	<input
		{...restProps}
		{id}
		{type}
		{placeholder}
		bind:this={element}
		bind:value
		oninput={handleInput}
		class="rounded-lg border border-zinc-800 bg-zinc-950 px-3 py-2 text-sm text-zinc-200 placeholder-zinc-600 scheme-dark transition-colors duration-100 focus:border-cyan-500 focus:outline-none {className}"
	/>
</div>
