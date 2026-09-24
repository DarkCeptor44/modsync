<script lang="ts">
	import './layout.css';
	import { onMount } from 'svelte';

	import NavBar from '../lib/components/NavBar.svelte';

	onMount(() => {
		if (import.meta.env.DEV) {
			console.log('in dev mode');
		}
	});

	let { children } = $props();
</script>

<div
	class="flex min-h-screen flex-col bg-zinc-950 font-sans text-zinc-100 antialiased selection:bg-cyan-500/20 selection:text-cyan-200"
>
	<NavBar />

	<main class="mx-auto flex w-full max-w-7xl flex-1 flex-col justify-center px-4 py-2 sm:px-6">
		{@render children()}
	</main>
</div>
