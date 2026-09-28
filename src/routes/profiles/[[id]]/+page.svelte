<script lang="ts">
	import { appState } from '$lib/api.svelte';
	import { toast } from '$lib/toast.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { t } from '$lib/i18n/index.svelte';

	import Button from '$lib/components/Button.svelte';
	import InputField from '$lib/components/InputField.svelte';
	import ExclusionForm from '$lib/components/ExclusionForm.svelte';

	const profileId = $derived(page.params.id);
	const isEditing = $derived(Boolean(profileId));

	let name = $state('');
	let source = $state('');
	let destination = $state('');
	let syncExclusions = $state<string[]>([]);
	let newSyncExclusion = $state('');
	let deleteExclusions = $state<string[]>([]);
	let newDeleteExclusion = $state('');

	let submitting = $state(false);

	const isValid = $derived(name && source && destination && !submitting);
	const hasAnyFields = $derived(
		name ||
			source ||
			destination ||
			syncExclusions.length ||
			newSyncExclusion ||
			deleteExclusions.length ||
			newDeleteExclusion
	);

	$effect(() => {
		if (isEditing) {
			const profile = appState.profiles.find((p) => p.id === profileId);
			if (!profile) {
				goto('/');
				return;
			}

			name = profile.name;
			source = profile.source;
			destination = profile.destination;
			syncExclusions = profile.syncExclusions || [];
			deleteExclusions = profile.deleteExclusions || [];
		} else {
			name = '';
			source = '';
			destination = '';
			syncExclusions = [];
			deleteExclusions = [];
		}
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!isValid) return;
		if (submitting) return;

		submitting = true;
		try {
			if (isEditing) {
				const newId = await appState.editProfile({
					id: profileId ?? '',
					name: name.trim(),
					source: source.trim(),
					destination: destination.trim(),
					syncExclusions,
					deleteExclusions
				});

				toast.show(t('form.updated'));

				if (profileId !== newId) {
					goto(`/profiles/${newId}`);
				}
			} else {
				await appState.addProfile({
					name: name.trim(),
					source: source.trim(),
					destination: destination.trim(),
					syncExclusions,
					deleteExclusions
				});

				toast.show(t('form.added'));
				goto('/');
			}
		} catch (err) {
			toast.show(
				typeof err === 'string' ? err : isEditing ? t('form.failedUpdate') : t('form.failedAdd'),
				'error'
			);
		} finally {
			submitting = false;
		}
	}

	async function handleDelete() {
		if (!isEditing || !profileId) return;

		console.log('deleting', profileId);
		// goto('/'); // TODO do delete
	}

	function handleCancel() {
		name = '';
		source = '';
		destination = '';
		syncExclusions = [];
		newSyncExclusion = '';
		deleteExclusions = [];
		newDeleteExclusion = '';

		if (isEditing) {
			goto('/');
		}
	}
</script>

<div class="mx-auto max-w-7xl space-y-6 sm:min-w-140 md:min-w-2xl">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-xl font-bold text-zinc-100">
				{isEditing ? t('form.titleEdit') : t('form.titleCreate')}
			</h1>
			{#if isEditing && profileId}
				<p class="font-mono text-xs text-zinc-500">ID: {profileId}</p>
			{/if}
		</div>
		<a href="/" class="text-xs text-zinc-400 transition-colors duration-100 hover:text-zinc-200"
			>&larr; {t('common.back')}</a
		>
	</div>

	<form
		onsubmit={handleSubmit}
		class="flex flex-col gap-2 rounded-xl border border-zinc-800 bg-zinc-900/60 p-6"
	>
		<InputField
			id="profile-name"
			label={t('form.nameLabel')}
			bind:value={name}
			placeholder="PAYDAY 2"
			required
			autocomplete="off"
		/>

		<div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
			<InputField
				id="source-path"
				label={t('form.sourceLabel')}
				bind:value={source}
				placeholder="D:\Mods\Payday2"
				required
				autocomplete="off"
			/>

			<InputField
				id="dest-path"
				label={t('form.destinationLabel')}
				bind:value={destination}
				placeholder="C:\Program Files\...\PAYDAY 2\mods"
				required
				autocomplete="off"
			/>
		</div>

		<ExclusionForm
			id="sync-exclusions"
			label={t('form.exclusionsSyncLabel')}
			bind:newExclusion={newSyncExclusion}
			bind:exclusions={syncExclusions}
		/>

		<ExclusionForm
			id="delete-exclusions"
			label={t('form.exclusionsDeleteLabel')}
			bind:newExclusion={newDeleteExclusion}
			bind:exclusions={deleteExclusions}
		/>

		<div class="mt-3 flex items-center justify-between gap-2">
			{#if isEditing}
				<Button variant="danger" onclick={handleDelete}>{t('form.delete')}</Button>
			{:else}
				<div></div>
			{/if}

			<div class="flex gap-3">
				{#if hasAnyFields}
					<Button variant="secondary" onclick={handleCancel}>{t('common.cancel')}</Button>
				{/if}

				<Button type="submit" disabled={!isValid}>
					{isEditing ? t('form.editButton') : t('form.addButton')}
				</Button>
			</div>
		</div>
	</form>
</div>
