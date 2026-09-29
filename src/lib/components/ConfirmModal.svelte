<script lang="ts">
	import { confirm } from '$lib/confirm.svelte';

	import Button from './Button.svelte';

	const { isOpen, options } = $derived(confirm.state);
</script>

{#if isOpen}
	<div
		class="fixed inset-0 z-9999 flex items-center justify-center bg-zinc-950/80 p-4 backdrop-blur-sm"
		onclick={() => confirm.cancel()}
		role="presentation"
	>
		<!-- svelte-ignore a11y_click_events_have_key_events -->
		<div
			class="w-full max-w-md space-y-5 rounded-xl border border-zinc-800 bg-zinc-950/95 p-5 shadow-2xl backdrop-blur-md outline-none"
			onclick={(e) => e.stopPropagation()}
			role="dialog"
			aria-modal="true"
			tabindex="-1"
		>
			<div class="flex items-start gap-3.5">
				<div
					class="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg border {options.variant ===
					'danger'
						? 'border-rose-500/30 bg-rose-500/10 text-rose-400'
						: 'border-amber-500/30 bg-amber-500/10 text-amber-400'}"
				>
					<svg
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
						stroke-width="2"
						stroke="currentColor"
						class="h-5 w-5"
					>
						<path
							stroke-linecap="round"
							stroke-linejoin="round"
							d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
						/>
					</svg>
				</div>

				<div class="space-y-1">
					<h3 class="text-sm font-semibold text-zinc-100">
						{options.title}
					</h3>
					<p class="text-xs leading-relaxed text-zinc-400">
						{options.message}
						{#if options.itemName}
							<span
								class="rounded border border-zinc-800 bg-zinc-900 px-1.5 py-0.5 font-mono text-[11px] font-medium text-cyan-400"
								>{options.itemName}</span
							> ?
						{/if}
					</p>
				</div>
			</div>

			<div class="flex items-center justify-end gap-2 border-t border-zinc-900 pt-2">
				<Button variant="secondary" onclick={() => confirm.cancel()}>
					{options.cancelText}
				</Button>
				<Button
					variant={options.variant === 'danger' ? 'danger' : 'primary'}
					onclick={() => confirm.confirm()}
				>
					{options.confirmText}
				</Button>
			</div>
		</div>
	</div>
{/if}
