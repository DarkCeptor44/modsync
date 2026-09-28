<script lang="ts">
	import { appState } from '$lib/api.svelte';
	import { onMount } from 'svelte';
	import { toast } from '$lib/toast.svelte';
	import { DEV } from '$lib/utils';
	import { t } from '$lib/i18n/index.svelte';

	import InputField from '$lib/components/InputField.svelte';
	import Button from '$lib/components/Button.svelte';

	let submitting = $state(false);
	let jobs = $state<number>(0);

	onMount(() => {
		appState.fetchSettings();

		if (DEV) {
			console.log('settings:', $state.snapshot(appState.settings));
		}
	});

	$effect(() => {
		jobs = appState.settings.jobs;
	});

	const isDirty = $derived.by(() => {
		return appState.settings.jobs !== jobs;
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		if (submitting) return;

		submitting = true;
		try {
			await appState.saveSettings({ jobs });
			toast.show(t('settings.saved'));
		} catch (err) {
			toast.show(typeof err === 'string' ? err : t('settings.failed'), 'error');
		} finally {
			submitting = false;
		}
	}

	function handleCancel() {
		if (!isDirty) return;

		jobs = appState.settings.jobs;
	}
</script>

<div class="mx-auto max-w-7xl space-y-6 sm:min-w-140 md:min-w-2xl">
	<div>
		<h1 class="text-xl font-bold text-zinc-100">{t('settings.title')}</h1>
		<p class="text-xs text-zinc-400">{t('settings.subtitle')}</p>
	</div>

	<form
		onsubmit={handleSubmit}
		class="flex flex-col gap-4 rounded-xl border border-zinc-800 bg-zinc-900/60 p-6"
	>
		<div class="grid grid-cols-1 gap-6">
			<InputField
				id="jobs"
				type="number"
				row
				label={t('settings.jobsLabel')}
				subLabel={t('settings.jobsSubtitle')}
				bind:value={jobs}
				placeholder="e.g. 4"
				min="1"
				max="1024"
				class="text-center"
			/>
		</div>

		<div class="mt-3 flex justify-end gap-3">
			{#if isDirty}
				<Button variant="secondary" onclick={handleCancel} disabled={submitting}
					>{t('common.cancel')}</Button
				>
			{/if}
			<Button type="submit" disabled={submitting || !isDirty}
				>{submitting ? t('common.saving') : t('settings.save')}</Button
			>
		</div>
	</form>
</div>
