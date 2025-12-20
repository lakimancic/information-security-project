<script lang="ts">
	import Welcome from '$lib/pages/Welcome.svelte';

	import { route } from '$lib/router.svelte';
</script>

<main class="h-screen bg-bg-0 text-fg-0">
	{#if route.path === '/'}
		<Welcome />
	{/if}
</main>