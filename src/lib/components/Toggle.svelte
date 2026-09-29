<script lang="ts">
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import type { Snippet } from 'svelte';

	interface Props extends HTMLButtonAttributes {
		id?: string;
		disabled?: boolean;
		checked: boolean;
		checkedColorClass?: string;
		uncheckedColorClass?: string;
		textClass?: string;
		children: Snippet;
	}

	let {
		id,
		disabled = false,
		checked = $bindable(false),
		class: className = '',
		checkedColorClass = 'bg-cyan-600',
		uncheckedColorClass = 'bg-zinc-700',
		textClass = 'text-sm',
		children,
		...restProps
	}: Props = $props();

	const newId = $derived(`toggle_${id}`);

	$effect(() => {
		if (!id || typeof window === 'undefined') return;

		const stored = localStorage.getItem(newId);
		if (stored !== null) {
			checked = stored === 'true';
		}
	});

	$effect(() => {
		if (!id || typeof window === 'undefined') return;

		localStorage.getItem(newId);
		localStorage.setItem(newId, String(checked));
	});

	function handleClick() {
		if (disabled) return;
		checked = !checked;
	}
</script>

<button
	{...restProps}
	{id}
	type="button"
	role="switch"
	aria-checked={checked}
	onclick={handleClick}
	{disabled}
	class="group flex items-center justify-between gap-3 rounded-lg border border-zinc-800 bg-zinc-900/80 px-3.5 py-2 font-medium transition-colors duration-100 enabled:cursor-pointer enabled:hover:border-zinc-700 disabled:cursor-not-allowed disabled:opacity-50 {textClass} {className}"
>
	<span class="text-zinc-300 transition-colors duration-100 enabled:group-hover:text-zinc-100">
		{@render children()}
	</span>

	<span
		class="relative inline-flex h-5 w-9 shrink-0 rounded-full border-2 border-transparent transition-colors duration-100 ease-in-out focus:outline-none enabled:cursor-pointer {checked
			? checkedColorClass
			: uncheckedColorClass}"
	>
		<span
			class="pointer-events-none inline-block h-4 w-4 transform rounded-full bg-zinc-950 shadow-md ring-0 transition duration-100 ease-in-out {checked
				? 'translate-x-4'
				: 'translate-x-0'}"
		></span>
	</span>
</button>
