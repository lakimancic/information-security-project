<script lang="ts">
	import { HardDrive} from '@lucide/svelte';
	import { Router } from '@lucide/svelte';
	import { navigate} from '$lib/router.svelte';
</script>

<div class="flex flex-col h-full w-full justify-center items-center">
	<div class="text-9xl font-black tracking-[1rem] p-6">Kript<span class="text-primary">Ox</span></div>
	<div class="text-3xl mt-10 text-fg-4">Welcome to <span class="italic">KriptOx</span>, encryption tool to keep your files safe.</div>
	<div class="text-3xl mt-5 text-fg-4">Where are the files you want to encrypt?</div>
	<div class="flex mt-20 gap-10">
		<button
			class="flex items-center gap-2 bg-primary text-bg-0 dark:text-fg-0 px-5 py-3 text-2xl font-bold cursor-pointer rounded-md"
			onclick={() => navigate("/local")}
		>
			<HardDrive class="size-7" /> Local File System
		</button>
		<button
			class="flex items-center gap-2 bg-primary text-bg-0 dark:text-fg-0 px-5 py-3 text-2xl font-bold cursor-pointer rounded-md"
			onclick={() => navigate("/remote")}
		>
			<Router class="size-7" />Remote Computer
		</button>
	</div>
</div>