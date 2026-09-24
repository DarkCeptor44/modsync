<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';

	import NavBar from '$lib/components/NavBar.svelte';

	onMount(() => {
		if (import.meta.env.DEV) {
			console.log('in dev mode');
		}
	});

	let { children } = $props();
</script>

<NavBar />

<main class="mx-auto flex w-full max-w-7xl flex-1 flex-col justify-start px-4 py-6 sm:px-6">
	{@render children()}
</main>
