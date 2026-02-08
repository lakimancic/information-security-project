<script lang="ts">
	import { Tooltip } from 'bits-ui';
	import { route, navigate } from '$lib/router.svelte';
	import { HardDrive, KeyRound, Router, LockKeyhole } from '@lucide/svelte';

	const navItems = [
		{
			id: 0,
			icon: HardDrive,
			name: 'Local File System',
			path: '/local'
		},
		{
			id: 1,
			icon: Router,
			name: 'Remote Computer',
			path: '/remote'
		},
		{
			id: 2,
			icon: KeyRound,
			name: 'Key Manager',
			path: '/keys'
		}
	];
</script>

<nav class="bg-bg-0 border-bg-2 flex h-full flex-col items-center gap-2 border">
	<LockKeyhole class="text-primary box-content size-8 p-3 opacity-50" />
	<Tooltip.Provider>
		{#each navItems as navItem (navItem.id)}
			<Tooltip.Root delayDuration={200}>
				{@const IconComponent = navItem.icon}
				<Tooltip.Trigger
					class="hover:bg-bg-2 border-r-4 border-transparent {route.path ===
						navItem.path && 'border-r-primary'} cursor-pointer"
					onclick={() => navigate(navItem.path)}
				>
					<IconComponent class="text-fg-3 box-content size-12 p-3" />
				</Tooltip.Trigger>
				<Tooltip.Content
					class="bg-bg-1 text-fg-3 border-bg-5 text-md z-10 ml-2 rounded-md border px-4 py-3"
					side="right"
				>
					{navItem.name}
				</Tooltip.Content>
			</Tooltip.Root>
		{/each}
	</Tooltip.Provider>
</nav>
