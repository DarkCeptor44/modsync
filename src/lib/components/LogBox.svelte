<script lang="ts">
	import type { SyncAction, SyncOutcome } from '$lib/types';
	import { humanBytes } from '$lib/utils';
	import { t } from '$lib/i18n/index.svelte';

	export type SystemLogLevel = 'info' | 'success' | 'warning' | 'error';

	export type Log =
		| { kind: 'outcome'; outcome: SyncOutcome }
		| { kind: 'system'; level: SystemLogLevel; message: string };

	interface Props {
		id: string;
		label?: string;
		dryRun?: boolean;
		logs: Log[];
		showSkipped?: boolean;
		class?: string;
		element?: HTMLDivElement | null;
	}

	let {
		id,
		label,
		dryRun = false,
		logs,
		showSkipped = false,
		class: className = '',
		element: containerEl = $bindable(null)
	}: Props = $props();

	let sortedLogs = $derived.by(() => {
		const result: Log[] = [];
		let outcomeBucket: { kind: 'outcome'; outcome: SyncOutcome }[] = [];

		const flushBucket = () => {
			if (outcomeBucket.length) {
				outcomeBucket.sort((a, b) => a.outcome.entry.localeCompare(b.outcome.entry));
				result.push(...outcomeBucket);
				outcomeBucket = [];
			}
		};

		for (const log of logs) {
			if (log.kind === 'system') {
				flushBucket();
				result.push(log);
			} else {
				outcomeBucket.push(log);
			}
		}

		flushBucket();

		return result;
	});

	$effect(() => {
		if (logs.length && containerEl) {
			containerEl.scrollTop = containerEl.scrollHeight;
		}
	});

	function getActionDetails(action: SyncAction): {
		badge: string;
		badgeClass: string;
		textClass: string;
		meta?: string;
	} | null {
		if (typeof action === 'string') {
			if (action === 'RemovedDir') {
				return {
					badge: 'REM DIR',
					badgeClass: 'border-amber-500/30 bg-amber-500/10 text-amber-400',
					textClass: 'text-zinc-300'
				};
			}

			return showSkipped
				? {
						badge: 'SKIP',
						badgeClass: 'border-zinc-700 bg-zinc-800/60 text-zinc-400',
						textClass: 'text-zinc-500'
					}
				: null;
		}

		if ('Copied' in action) {
			return {
				badge: 'ADD',
				badgeClass: 'border-cyan-500/30 bg-cyan-500/10 text-cyan-400',
				textClass: 'text-zinc-100',
				meta: humanBytes(action.Copied.bytes)
			};
		}

		if ('Removed' in action) {
			return {
				badge: 'DEL',
				badgeClass: 'border-rose-500/30 bg-rose-500/10 text-rose-400',
				textClass: 'text-zinc-300',
				meta: humanBytes(action.Removed.bytes)
			};
		}

		if ('Failed' in action) {
			return {
				badge: 'ERR',
				badgeClass: 'border-rose-500/50 bg-rose-500/20 text-rose-300 font-bold',
				textClass: 'text-rose-300',
				meta: action.Failed.error
			};
		}

		return {
			badge: 'INFO',
			badgeClass: 'border-zinc-700 bg-zinc-800 text-zinc-300',
			textClass: 'text-zinc-300'
		};
	}

	function getSystemLevelDetails(level: SystemLogLevel) {
		switch (level) {
			case 'success':
				return {
					badge: 'OK',
					badgeClass: 'border-emerald-500/30 bg-emerald-500/10 text-emerald-400',
					textClass: 'text-emerald-300 font-medium'
				};
			case 'warning':
				return {
					badge: 'WARN',
					badgeClass: 'border-amber-500/30 bg-amber-500/10 text-amber-400',
					textClass: 'text-amber-300 font-medium'
				};
			case 'error':
				return {
					badge: 'ERR',
					badgeClass: 'border-rose-500/50 bg-rose-500/20 text-rose-300 font-bold',
					textClass: 'text-rose-300 font-medium'
				};
			case 'info':
			default:
				return {
					badge: 'INFO',
					badgeClass: 'border-cyan-500/30 bg-cyan-500/10 text-cyan-400',
					textClass: 'text-zinc-200'
				};
		}
	}
</script>

<div class="flex w-full flex-col gap-1.5">
	{#if label}
		<div class="flex items-center justify-between">
			<label for={id} class="text-xs font-medium text-zinc-400">
				{label}
			</label>
			{#if dryRun}
				<span
					class="rounded border border-amber-500/30 bg-amber-500/10 px-2 py-0.5 font-mono text-[10px] font-semibold tracking-wider text-amber-400 uppercase"
				>
					{t('sync.dryRunActive')}
				</span>
			{/if}
		</div>
	{/if}

	<div
		{id}
		bind:this={containerEl}
		class="relative flex h-70 w-full flex-col gap-1 overflow-y-auto rounded-lg border border-zinc-800 bg-zinc-950 p-3 font-mono text-xs scheme-dark transition-colors duration-100 {className}"
	>
		{#if sortedLogs.length === 0}
			<div class="flex h-full items-center justify-center text-zinc-600 select-none">
				{t('sync.logsPlaceholder')}
			</div>
		{:else}
			{#each sortedLogs as log}
				{#if log.kind === 'system'}
					{@const sys = getSystemLevelDetails(log.level)}

					<div
						class="group flex items-center gap-3 rounded px-1.5 py-0.5 transition-colors duration-100 hover:bg-zinc-900/60"
					>
						<span
							class="shrink-0 rounded border px-1.5 py-px text-[10px] font-bold tracking-wider uppercase {sys.badgeClass}"
						>
							{sys.badge}
						</span>
						<span class="truncate {sys.textClass}" title={log.message}>
							{log.message}
						</span>
					</div>
				{:else}
					{@const details = getActionDetails(log.outcome.action)}

					{#if details}
						<div
							class="group flex items-center justify-between gap-3 rounded px-1.5 py-0.5 transition-colors duration-100 hover:bg-zinc-900/60"
						>
							<div class="flex min-w-0 items-center gap-2">
								<span
									class="shrink-0 rounded border px-1.5 py-px text-[10px] font-bold tracking-wider uppercase {details.badgeClass}"
								>
									{details.badge}
								</span>
								<span class="truncate {details.textClass}" title={log.outcome.entry}>
									{log.outcome.entry}
								</span>
							</div>

							{#if details.meta}
								<span class="shrink-0 text-[11px] text-zinc-500 group-hover:text-zinc-400">
									{details.meta}
								</span>
							{/if}
						</div>
					{/if}
				{/if}
			{/each}
		{/if}
	</div>
</div>
