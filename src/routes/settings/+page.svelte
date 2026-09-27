<script lang="ts">
	import { appState } from '$lib/api.svelte';
	import { onMount } from 'svelte';
	import { toast } from '$lib/toast.svelte';
	import { DEV } from '$lib/utils';

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
			toast.show('Settings saved');
		} catch (err) {
			toast.show(typeof err === 'string' ? err : 'Failed to save settings', 'error');
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
		<h1 class="text-xl font-bold text-zinc-100">Settings</h1>
		<p class="text-xs text-zinc-400">Manage global settings</p>
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
				label="Parallel Jobs"
				subLabel="Number of parallel operations. Defaults to 75% of available CPU cores, but higher values can be tried at your own risk of slowing down the computer."
				bind:value={jobs}
				placeholder="e.g. 4"
				min="1"
				max="1024"
				class="text-center"
			/>
		</div>

		<div class="mt-3 flex justify-end gap-3">
			{#if isDirty}
				<Button variant="secondary" onclick={handleCancel} disabled={submitting}>Cancel</Button>
			{/if}
			<Button type="submit" disabled={submitting || !isDirty}
				>{submitting ? 'Saving...' : 'Save Settings'}</Button
			>
		</div>
	</form>
</div>
