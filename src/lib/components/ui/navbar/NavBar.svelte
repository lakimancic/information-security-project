<script lang="ts">
	import { Tooltip } from "bits-ui";
	import { route, navigate } from '$lib/router.svelte';
	import { HardDrive, KeyRound, Router, LockKeyhole } from '@lucide/svelte';

	const navItems = [
		{
			id: 0,
			icon: HardDrive,
			name: "Local File System",
			path: "/local"
		},
		{
			id: 1,
			icon: Router,
			name: "Remote Computer",
			path: "/remote"
		},
		{
			id: 2,
			icon: KeyRound,
			name: "Key Manager",
			path: "/keys"
		}
	];
</script>

<nav class="bg-bg-0 h-full flex flex-col items-center border border-bg-2 gap-2">
	<LockKeyhole class="size-8 p-3 box-content text-primary opacity-50" />
	<Tooltip.Provider>
		{#each navItems as navItem(navItem.id)}
			<Tooltip.Root delayDuration={200}>
				{@const IconComponent = navItem.icon}
				<Tooltip.Trigger
					class="hover:bg-bg-2 border-transparent border-r-4 {route.path === navItem.path && 'border-r-primary'} cursor-pointer"
					onclick={() => navigate(navItem.path)}
				>
					<IconComponent class="size-12 p-3 box-content text-fg-3" />
				</Tooltip.Trigger>
				<Tooltip.Content
					class="z-10 bg-bg-1 text-fg-3 px-4 py-3 ml-2 border border-bg-5 text-md rounded-md"
					side="right"
				>
					{navItem.name}
				</Tooltip.Content>
			</Tooltip.Root>
		{/each}
	</Tooltip.Provider>
</nav>