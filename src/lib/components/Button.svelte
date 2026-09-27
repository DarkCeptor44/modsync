<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLAnchorAttributes, HTMLButtonAttributes } from 'svelte/elements';

	type Variant = 'primary' | 'secondary' | 'danger';

	interface Props {
		href?: string;
		variant?: Variant;
		type?: HTMLButtonAttributes['type'];
		children: Snippet;
		onclick?: (e: MouseEvent) => void;
		class?: string;
		textSize?: string;
		disabled?: boolean;
	}

	let {
		href,
		variant = 'primary',
		type = 'button',
		children,
		onclick,
		class: className = '',
		textSize = 'text-xs',
		disabled = false,
		...restProps
	}: Props = $props();

	const variants: Record<Variant, string> = $derived({
		primary: 'bg-cyan-600 text-zinc-950 hover:bg-cyan-500',
		secondary: 'bg-zinc-800 text-zinc-200 border border-zinc-700/50 hover:bg-zinc-700',
		danger: 'bg-rose-950/40 text-rose-300 border border-rose-800/50 hover:bg-rose-900/50'
	});
	const baseStyles = $derived(
		`inline-flex items-center justify-center rounded-lg px-4 py-2 font-semibold transition-all duration-100 select-none focus:outline-none ${textSize} ${
			disabled
				? 'opacity-50 pointer-events-none cursor-not-allowed'
				: 'cursor-pointer active:scale-98'
		}`
	);

	function handleAnchorClick(e: MouseEvent) {
		if (disabled) {
			e.preventDefault();
			e.stopPropagation();
			return;
		}
		onclick?.(e);
	}
</script>

{#if href && !disabled}
	<a
		{href}
		onclick={handleAnchorClick}
		class="{baseStyles} {variants[variant]} {className}"
		{...restProps as HTMLAnchorAttributes}
	>
		{@render children()}
	</a>
{:else if href && disabled}
	<span
		role="link"
		aria-disabled="true"
		class="{baseStyles} {variants[variant]} {className}"
		{...restProps}
	>
		{@render children()}
	</span>
{:else}
	<button
		{type}
		{onclick}
		{disabled}
		class="{baseStyles} {variants[variant]} {className}"
		{...restProps as HTMLButtonAttributes}
	>
		{@render children()}
	</button>
{/if}
