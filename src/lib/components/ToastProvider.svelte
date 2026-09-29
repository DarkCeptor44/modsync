<!--
   This Source Code Form is subject to the terms of the Mozilla Public
   License, v. 2.0. If a copy of the MPL was not distributed with this
   file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

<script lang="ts">
	import { toast } from '$lib/toast.svelte';
	import { fly } from 'svelte/transition';
</script>

<div
	class="pointer-events-none fixed top-4 left-1/2 z-9980 flex w-full max-w-sm -translate-x-1/2 flex-col gap-2 px-4"
>
	{#each toast.items as item (item.id)}
		{@const isSuccess = item.type === 'success'}
		{@const isError = item.type === 'error'}

		<div
			transition:fly={{ y: -12, duration: 150 }}
			class="pointer-events-auto flex items-center justify-between rounded-xl border bg-zinc-900/95 p-3 text-xs shadow-2xl backdrop-blur-md {isSuccess
				? 'border-emerald-500/30 text-zinc-100 shadow-emerald-950/20'
				: isError
					? 'border-rose-500/30 text-zinc-100 shadow-rose-950/20'
					: 'border-cyan-500/30 text-zinc-100 shadow-cyan-950/20'}"
		>
			<div class="flex min-w-0 items-center gap-2.5">
				<div
					class="flex h-6 w-6 shrink-0 items-center justify-center rounded-lg border transition-colors duration-100 {isSuccess
						? 'border-emerald-500/30 bg-emerald-500/10 text-emerald-400'
						: isError
							? 'border-rose-500/30 bg-rose-500/10 text-rose-400'
							: 'border-cyan-500/30 bg-cyan-500/10 text-cyan-400'}"
				>
					{#if isSuccess}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="2.5"
							stroke="currentColor"
							class="h-3.5 w-3.5"
						>
							<path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
						</svg>
					{:else if isError}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="2.5"
							stroke="currentColor"
							class="h-3.5 w-3.5"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M12 9v3.75m9-.75a9 9 0 11-18 0 9 9 0 0118 0zm-9 3.75h.008v.008H12v-.008z"
							/>
						</svg>
					{:else}
						<svg
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
							stroke-width="2.5"
							stroke="currentColor"
							class="h-3.5 w-3.5"
						>
							<path
								stroke-linecap="round"
								stroke-linejoin="round"
								d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"
							/>
						</svg>
					{/if}
				</div>

				<span class="font-medium wrap-break-word text-zinc-200">
					{item.message}
				</span>
			</div>

			<button
				type="button"
				onclick={() => toast.dismiss(item.id)}
				title="Dismiss"
				aria-label="Dismiss notification"
				class="ml-3 shrink-0 cursor-pointer rounded-lg p-1 text-zinc-400 transition-colors duration-100 hover:bg-zinc-800 hover:text-zinc-200 focus:outline-none active:scale-98"
			>
				<svg
					xmlns="http://www.w3.org/2000/svg"
					fill="none"
					viewBox="0 0 24 24"
					stroke-width="2"
					stroke="currentColor"
					class="h-3.5 w-3.5"
				>
					<path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
				</svg>
			</button>
		</div>
	{/each}
</div>
