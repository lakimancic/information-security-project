<script lang="ts">
	import Background from '$lib/components/ui/background/Background.svelte';
	import Welcome from '$lib/pages/Welcome.svelte';

	import { route } from '$lib/router.svelte';
	import NavBar from '$lib/components/ui/navbar/NavBar.svelte';
	import ThemeToggle from '$lib/components/ui/theme-toggle/ThemeToggle.svelte';
	import LocalFileSystem from '$lib/pages/LocalFileSystem.svelte';
	import NetworkTransfer from '$lib/pages/NetworkTransfer.svelte';
</script>

<main class="h-screen text-fg-0 flex overflow-hidden">
	<Background />
	{#if route.path === '/'}
		<Welcome />
	{:else}
		<NavBar />
		<div class="flex flex-col flex-1 backdrop-blur-md min-h-0">
			<div class="flex h-14 items-center justify-between">
				<div class="text-3xl font-extrabold tracking-widest mx-5">
					Kript<span class="text-primary">Ox</span>
				</div>
				<ThemeToggle className="mr-3"/>
			</div>
			{#if route.path === '/local'}
				<LocalFileSystem />
			{/if}
			{#if route.path === '/remote'}
				<NetworkTransfer />
			{/if}
		</div>
	{/if}
</main>