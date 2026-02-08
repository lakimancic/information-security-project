<script lang="ts">
	import Background from '$lib/components/ui/background/Background.svelte';
	import Welcome from '$lib/pages/Welcome.svelte';

	import { route } from '$lib/router.svelte';
	import NavBar from '$lib/components/ui/navbar/NavBar.svelte';
	import ThemeToggle from '$lib/components/ui/theme-toggle/ThemeToggle.svelte';
	import LocalFileSystem from '$lib/pages/LocalFileSystem.svelte';
	import NetworkTransfer from '$lib/pages/NetworkTransfer.svelte';
	import KeyManager from '$lib/pages/KeyManager.svelte';
	import Notifications from '$lib/components/ui/notifications/Notifications.svelte';
</script>

<main class="text-fg-0 relative flex h-screen overflow-hidden">
	<Background />
	<Notifications />
	{#if route.path === '/'}
		<Welcome />
	{:else}
		<NavBar />
		<div class="flex min-h-0 flex-1 flex-col backdrop-blur-md">
			<div class="flex h-14 items-center justify-between">
				<div class="mx-5 text-3xl font-extrabold tracking-widest">
					Kript<span class="text-primary">Ox</span>
				</div>
				<ThemeToggle className="mr-3" />
			</div>
			{#if route.path === '/local'}
				<LocalFileSystem />
			{/if}
			{#if route.path === '/remote'}
				<NetworkTransfer />
			{/if}
			{#if route.path === '/keys'}
				<KeyManager />
			{/if}
		</div>
	{/if}
</main>
