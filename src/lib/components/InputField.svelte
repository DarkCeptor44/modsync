<!--
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import type { HTMLInputAttributes } from 'svelte/elements';

	interface Props extends HTMLInputAttributes {
		label?: string;
		subLabel?: string;
		id: string;
		type?: 'text' | 'number' | 'time' | 'date' | 'url' | 'password';
		placeholder?: string;
		value: string | number | null;
		required?: boolean;
		row?: boolean;
		prefix?: string;
		transform?: (value: string) => string;
		element?: HTMLInputElement | null;
	}

	let {
		label,
		subLabel,
		id,
		type = 'text',
		placeholder = '',
		value = $bindable(),
		required = false,
		row = false,
		prefix,
		transform,
		element = $bindable(null),
		class: className = '',
		oninput,
		...restProps
	}: Props = $props();

	function handleInput(e: Event & { currentTarget: HTMLInputElement }) {
		let rawValue = e.currentTarget.value;

		if (transform) {
			rawValue = transform(rawValue);
			e.currentTarget.value = rawValue;
		}

		if (type === 'number') {
			if (rawValue.trim() === '') {
				value = null;
			} else {
				const parsed = Number(rawValue);
				value = Number.isNaN(parsed) ? null : parsed;
			}
		} else {
			value = rawValue;
		}

		if (typeof oninput === 'function') {
			oninput(e);
		}
	}
</script>

<div class="flex {row ? 'w-full flex-row items-center justify-between gap-6' : 'flex-col gap-1.5'}">
	{#if label || subLabel}
		<label for={id} class="flex flex-col gap-0.5 {row ? 'max-w-xl' : ''}">
			<div class="flex items-center gap-1">
				<span
					class="font-medium text-zinc-200 {row
						? 'text-sm font-semibold text-zinc-100'
						: 'text-xs text-zinc-400'}">{label}</span
				>
				{#if required}
					<span class="text-rose-400">*</span>
				{/if}
			</div>

			{#if row && subLabel}
				<span class="text-xs leading-relaxed text-zinc-400">{subLabel}</span>
			{/if}
		</label>
	{/if}

	<div class="relative flex items-center {row ? 'shrink-0' : 'w-full'}">
		<input
			{...restProps}
			{id}
			{type}
			{placeholder}
			bind:this={element}
			bind:value
			oninput={handleInput}
			class="w-full rounded-lg border border-zinc-800 bg-zinc-950 py-2 text-sm text-zinc-200 placeholder-zinc-600 scheme-dark transition-colors duration-100 enabled:focus:border-cyan-500 enabled:focus:outline-none {className} {type ===
				'number' && prefix
				? '[appearance:textfield] [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none'
				: ''} {type === 'number' && prefix ? 'pr-8 pl-3' : 'px-3'}"
		/>
		{#if prefix}
			<span class="pointer-events-none absolute right-3 text-xs font-semibold text-zinc-500"
				>{prefix}</span
			>
		{/if}
	</div>
</div>
