<script lang="ts">
	import { appState } from '$lib/api.svelte';
	import { toast } from '$lib/toast.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	import Button from '$lib/components/Button.svelte';
	import InputField from '$lib/components/InputField.svelte';

	const profileId = $derived(page.params.id);
	const isEditing = $derived(Boolean(profileId));

	let name = $state('');
	let source = $state('');
	let destination = $state('');
	let exclusions = $state<string[]>([]);
	let newExclusion = $state('');

	let submitting = $state(false);

	const isValid = $derived(name && source && destination && !submitting);
	const hasAnyFields = $derived(name || source || destination || exclusions.length || newExclusion);

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
			exclusions = profile.exclusions || [];
		} else {
			name = '';
			source = '';
			destination = '';
			exclusions = [];
		}
	});

	function addExclusion() {
		if (!newExclusion.trim()) return;
		if (newExclusion.trim()) {
			exclusions = [...exclusions, newExclusion.trim()];
			newExclusion = '';
		}
	}

	function removeExclusion(index: number) {
		exclusions = exclusions.filter((_, i) => i !== index);
	}

	async function handleSubmit(e: Event) {
		e.preventDefault();
		if (!isValid) return;
		if (submitting) return;

		submitting = true;
		try {
			await appState.addProfile({
				name: name.trim(),
				source: source.trim(),
				destination: destination.trim(),
				exclusions
			});

			toast.show('Profile added');
			goto('/');
		} catch (err) {
			console.error(err);
			toast.show(typeof err === 'string' ? err : 'Failed to add profile', 'error');
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
		exclusions = [];
		newExclusion = '';

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
			{#if isEditing}
				<p class="font-mono text-xs text-zinc-500">ID: {profileId}</p>
			{/if}
		</div>
		<a href="/" class="text-xs text-zinc-400 transition-colors duration-100 hover:text-zinc-200"
			>&larr; Back to Profiles</a
		>
	</div>

	<form
		onsubmit={handleSubmit}
		class="flex flex-col gap-4 rounded-xl border border-zinc-800 bg-zinc-900/60 p-6"
	>
		<div class="grid grid-cols-1 gap-2">
			<InputField
				id="profile-name"
				label="Profile Name"
				bind:value={name}
				placeholder="PAYDAY 2"
				required
				autocomplete="off"
			/>
		</div>
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

		<div class="flex items-end gap-2">
			<div class="flex-1">
				<InputField
					id="exclusion-input"
					label="Protected Paths (Exclusions)"
					bind:value={newExclusion}
					placeholder="base/, logs/, saves/"
					autocomplete="off"
				/>
			</div>
			<Button
				variant="secondary"
				onclick={addExclusion}
				class="h-9.5 px-4"
				disabled={!newExclusion.trim()}>Add</Button
			>
		</div>

		<div class="grid grid-cols-12 gap-2">
			<div class="mt-3 flex flex-wrap gap-2">
				{#each exclusions as item, i}
					<span
						class="inline-flex items-center gap-1.5 rounded-md border border-rose-800/40 bg-rose-950/30 px-2.5 py-1 font-mono text-xs text-rose-300"
					>
						{item}
						<button
							type="button"
							onclick={() => removeExclusion(i)}
							class="text-rose-400 hover:text-rose-200">&times;</button
						>
					</span>
				{/each}
			</div>
		</div>

		<div class="flex items-center justify-between gap-2">
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
