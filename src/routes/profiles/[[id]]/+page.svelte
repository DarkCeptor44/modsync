<script lang="ts">
	import { appState } from '$lib/api.svelte';
	import { toast } from '$lib/toast.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

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

				toast.show('Profile updated');

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

				toast.show('Profile added');
				goto('/');
			}
		} catch (err) {
			toast.show(
				typeof err === 'string'
					? err
					: isEditing
						? 'Failed to update profile'
						: 'Failed to add profile',
				'error'
			);
		} finally {
			submitting = false;
		}
	}

	async function handleDelete() {
		if (!isEditing || !profileId) return;

		console.log('deleting', profileId);
		// goto('/');
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
				{isEditing ? 'Edit Profile' : 'Create Profile'}
			</h1>
			{#if isEditing && profileId}
				<p class="font-mono text-xs text-zinc-500">ID: {profileId}</p>
			{/if}
		</div>
		<a href="/" class="text-xs text-zinc-400 transition-colors duration-100 hover:text-zinc-200"
			>&larr; Back to Profiles</a
		>
	</div>

	<form
		onsubmit={handleSubmit}
		class="flex flex-col gap-2 rounded-xl border border-zinc-800 bg-zinc-900/60 p-6"
	>
		<InputField
			id="profile-name"
			label="Profile Name"
			bind:value={name}
			placeholder="PAYDAY 2"
			required
			autocomplete="off"
		/>

		<div class="grid grid-cols-1 gap-2 sm:grid-cols-2">
			<InputField
				id="source-path"
				label="Source Path"
				bind:value={source}
				placeholder="D:\Mods\Payday2"
				required
				autocomplete="off"
			/>

			<InputField
				id="dest-path"
				label="Destination Path"
				bind:value={destination}
				placeholder="C:\Program Files\...\PAYDAY 2\mods"
				required
				autocomplete="off"
			/>
		</div>

		<ExclusionForm
			id="sync-exclusions"
			label="Protected Paths In Sync"
			bind:newExclusion={newSyncExclusion}
			bind:exclusions={syncExclusions}
		/>

		<ExclusionForm
			id="delete-exclusions"
			label="Protected Paths In Delete"
			bind:newExclusion={newDeleteExclusion}
			bind:exclusions={deleteExclusions}
		/>

		<div class="mt-3 flex items-center justify-between gap-2">
			{#if isEditing}
				<Button variant="danger" onclick={handleDelete}>Delete Profile</Button>
			{:else}
				<div></div>
			{/if}

			<div class="flex gap-3">
				{#if hasAnyFields}
					<Button variant="secondary" onclick={handleCancel}>Cancel</Button>
				{/if}

				<Button type="submit" disabled={!isValid}>
					{isEditing ? 'Save Changes' : 'Create Profile'}
				</Button>
			</div>
		</div>
	</form>
</div>
