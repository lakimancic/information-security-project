<script lang="ts">
	import { HardDrive } from '@lucide/svelte';
	import { Router } from '@lucide/svelte';
	import { navigate } from '$lib/router.svelte';
	import ThemeToggle from '$lib/components/ui/theme-toggle/ThemeToggle.svelte';
</script>

<div class="flex h-full w-full flex-col items-center justify-center">
	<div class="p-6 text-8xl font-black tracking-[1rem]">
		Kript<span class="text-primary">Ox</span>
	</div>
	<div class="relative flex flex-col items-center">
		<div
			class="bg-bg-0 absolute top-0 left-0 -z-10 h-full w-full blur-2xl"
		></div>
		<div class="text-fg-4 mt-10 text-2xl">
			Welcome to <span class="text-fg-2 italic">KriptOx</span>, encryption tool
			to keep your files safe.
		</div>
		<div class="text-fg-4 mt-3 text-2xl">
			Where are the files you want to encrypt?
		</div>
	</div>
	<div class="mt-10 flex gap-10">
		<button
			class="bg-primary text-bg-0 dark:text-fg-0 hover:bg-primary/80 flex cursor-pointer items-center gap-2 rounded-md px-5 py-3 text-2xl font-bold duration-300"
			onclick={() => navigate('/local')}
		>
			<HardDrive class="size-7" /> Local File System
		</button>
		<button
			class="bg-primary text-bg-0 dark:text-fg-0 hover:bg-primary/80 flex cursor-pointer items-center gap-2 rounded-md px-5 py-3 text-2xl font-bold duration-300"
			onclick={() => navigate('/remote')}
		>
			<Router class="size-7" />Remote Computer
		</button>
	</div>
	<ThemeToggle className="fixed top-2 right-2" />
</div>
