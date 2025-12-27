<script lang="ts">
	import type { LocalFile } from '$lib/components/ui/file-explorer/utils';
	import { sizeToString, typeToIcon } from '$lib/components/ui/file-explorer/utils';
	import { writable } from 'svelte/store';

	const {
		"class": className,
		files = [],
		pwd = '',
	} : {
		"class": string;
		files: LocalFile[];
		pwd: string;
	} = $props();

	const searchText = writable('');
</script>

<div class="flex flex-col {className} h-full min-h-0 box-border">
	<div class="bg-bg-2 p-2 text-sm text-primary font-bold">
		{pwd}
	</div>

	<div class="flex flex-col flex-1 min-h-0 w-full text-left">
		<div
			class="grid grid-cols-[2rem_2fr_2fr_1fr_2fr] bg-bg-1 text-sm font-medium shrink-0"
		>
			<div></div>
			<div class="px-2 py-2 truncate">Filename</div>
			<div class="px-4 py-2 truncate">Last Modified</div>
			<div class="px-4 py-2">Size</div>
			<div class="px-4 py-2">Type</div>
		</div>
		<div class="flex-1 min-h-0 overflow-y-auto">
			{#each files as file}
				<div
					class={`grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center select-none text-sm
					${file.selected
						? "bg-primary/20 border border-primary"
						: "hover:bg-bg-2/50"}`}
				>
					<div class="py-2 pl-2">
						<img
							src={typeToIcon(file.type ?? "")}
							class="h-6"
							alt="file-icon"
						/>
					</div>

					<div class="px-2 py-2 truncate">
						{file.filename}
					</div>

					<div class="px-4 py-2 truncate">
						{file.lastModified}
					</div>

					<div class="px-4 py-2 text-left">
						{file.size ? sizeToString(file.size) : ""}
					</div>

					<div class="px-4 py-2 truncate">
						{file.type ?? "Unknown"}
					</div>
				</div>
			{/each}
		</div>
	</div>
	<div class="bg-bg-2/70 shrink-0">
		<input
			type="text"
			placeholder="Search..."
			bind:value={$searchText}
			class="w-full p-2 border border-bg-4 rounded outline-none"
		/>
	</div>
</div>
