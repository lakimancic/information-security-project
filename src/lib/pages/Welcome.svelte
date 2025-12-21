<script lang="ts">
	import { HardDrive} from '@lucide/svelte';
	import { Router } from '@lucide/svelte';
	import { navigate} from '$lib/router.svelte';
	import ThemeToggle from '$lib/components/ui/theme-toggle/ThemeToggle.svelte';
</script>

<div class="flex flex-col h-full w-full justify-center items-center">
	<div class="text-8xl font-black tracking-[1rem] p-6">Kript<span class="text-primary">Ox</span></div>
	<div class="relative flex flex-col items-center">
		<div class="absolute w-full h-full blur-2xl bg-bg-0 left-0 top-0 -z-10"></div>
		<div class="text-2xl mt-10 text-fg-4">Welcome to <span class="italic text-fg-2">KriptOx</span>, encryption tool to keep your files safe.</div>
		<div class="text-2xl mt-3 text-fg-4">Where are the files you want to encrypt?</div>
	</div>
	<div class="flex mt-10 gap-10">
		<button
			class="flex items-center gap-2 bg-primary text-bg-0 dark:text-fg-0 px-5 py-3 text-2xl font-bold cursor-pointer rounded-md hover:opacity-80 duration-300"
			onclick={() => navigate("/local")}
		>
			<HardDrive class="size-7" /> Local File System
		</button>
		<button
			class="flex items-center gap-2 bg-primary text-bg-0 dark:text-fg-0 px-5 py-3 text-2xl font-bold cursor-pointer rounded-md hover:opacity-80 duration-300"
			onclick={() => navigate("/remote")}
		>
			<Router class="size-7" />Remote Computer
		</button>
	</div>
	<ThemeToggle className="fixed top-2 right-2"/>
</div>