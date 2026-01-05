<script lang="ts">
	import type { LocalFile } from '$lib/components/ui/file-explorer/utils';
	import { sizeToString, typeToIcon } from '$lib/components/ui/file-explorer/utils';
	import { writable } from 'svelte/store';
	import { ArrowDown, ArrowUp, RotateCwIcon, type IconProps } from '@lucide/svelte';
	import type { Component } from 'svelte';

	let {
		"class": className,
		files = [],
		pwd = $bindable(''),
		label,
		locked = $bindable(false),
		lockIcon : LockIcon,
		unlockIcon : UnlockIcon,
	} : {
		"class": string;
		files: LocalFile[];
		pwd: string;
		label: string;
		locked: boolean;
		lockIcon: Component<IconProps, {}>,
		unlockIcon: Component<IconProps, {}>
	} = $props();

	const searchText = writable('');
	let searchFilter = $state(new RegExp(""));
	let oldPwd = $state('');

	let searchBar : HTMLInputElement;
	let pwdBar : HTMLInputElement;
	let itemsMenu : HTMLDivElement;

	type SortColumn = 'name'|'date'|'size'|'type';
	let sortBy = writable<SortColumn>('name');
	let sortDirection = writable<'asc'|'desc'>('asc');

	const ArrowIcon = $derived($sortDirection === 'asc' ? ArrowUp : ArrowDown);

	const tableColumns = [
		{ title: "Filename", sortKey: "name" },
		{ title: "Last Modified", sortKey: "date" },
		{ title: "Size", sortKey: "size" },
		{ title: "Type", sortKey: "type" },
	];

	let selectedIndexStart = $state(-1);
	let selectedIndexEnd = $state(-1);
	let rowEls = $state<{ [key: number]: HTMLDivElement | null }>({});

	const filesWithBack = $derived.by(() => {
		const filesCopy = files.toSorted((a, b) => {
			let res = 0;
			if ($sortBy === 'size') res = (a.size ?? 0) - (b.size ?? 0);
			else if ($sortBy === 'date') res = (new Date(a.lastModified).getTime() - new Date(b.lastModified).getTime());
			else if ($sortBy === 'name') res = a.filename.localeCompare(b.filename);
			else res = a.typeLong?.localeCompare(b.typeLong ?? '') ?? 0;

			return res * ($sortDirection === 'asc' ? 1 : -1);
		}).filter(file => {
			return searchFilter.test(file.filename);
		});
		
		return [
			{ filename: "..", lastModified: "", type: "back", size: 0 },
			...filesCopy
		];
	});

	const pwdFocusIn = () => {
		oldPwd = pwd;
	};

	const pwdFocusOut = () => {
		pwd = oldPwd;
	};

	const handleColumnClick = (colName: SortColumn) => {
		if (colName === $sortBy) {
			sortDirection.update(old => old === 'asc' ? 'desc' : 'asc');
		} else {
			sortBy.update(() => colName);
		}
	};

	const handleKeyDown = (e: KeyboardEvent) => {
		e.preventDefault();
		if (e.key === 'ArrowUp') {
			selectedIndexEnd = selectedIndexEnd-1;
			if (!e.shiftKey) {
				selectedIndexStart = selectedIndexEnd;
			}
		} else if (e.key === 'ArrowDown') {
			selectedIndexEnd = selectedIndexEnd+1;
			if (!e.shiftKey) {
				selectedIndexStart = selectedIndexEnd;
			}
		} else if (e.key === '/') {
			searchBar.focus();
		} else if (e.key === '?') {
			pwdBar.focus();
		}
		selectedIndexStart = Math.min(Math.max(selectedIndexStart, 1), filesWithBack.length-1);
		selectedIndexEnd = Math.min(Math.max(selectedIndexEnd, 1), filesWithBack.length-1);
	};

	const handleItemClick = (fileIndex: number, e: MouseEvent) => {
		if (fileIndex === 0) return;

		if (e.shiftKey) {
			selectedIndexEnd = fileIndex;
		} else {
			selectedIndexStart = selectedIndexEnd = fileIndex;
		}
	};

	const escapeItemsMenu = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			e.preventDefault();
			itemsMenu.focus();
		}
	};

	const searchKeyDown = (e: KeyboardEvent) => {
		escapeItemsMenu(e);

		if (e.key === 'Enter') {
			selectedIndexStart = selectedIndexEnd = -1;
			searchFilter = new RegExp($searchText);
		}
	};

	$effect(() => {
		const i = selectedIndexEnd;
		if (i < 0) return;

		const row = rowEls[i === 1 ? 0 : i];
		if (!row) return;
		
		row.scrollIntoView({
			block: 'nearest',
			behavior: 'auto'
		});
	});
</script>

<div class="flex flex-col {className} h-full min-h-0 box-border">
	<p class="p-2 text-center text-xl text-fg-2">{label}</p>
	<div class="flex bg-bg-2 p-2 text-sm text-primary font-bold gap-2">
		<input
			type="text"
			bind:value={pwd}
			class="w-full border-none disabled:text-primary/50 outline-none"
			onfocusin={pwdFocusIn}
			onfocusout={pwdFocusOut}
			bind:this={pwdBar}
			onkeydown={escapeItemsMenu}
		/>
		<RotateCwIcon class="cursor-pointer text-fg-3 hover:text-fg-0 transition-colors duration-300" />
		{#if locked}
			<UnlockIcon class="cursor-pointer text-fg-2 hover:text-primary transition-colors duration-300" />
		{:else}
			<LockIcon class="cursor-pointer text-fg-2 hover:text-primary transition-colors duration-300" />
		{/if}
	</div>

	<div 
		class="flex flex-col flex-1 min-h-0 w-full text-left outline-none"
		role="menu"
		onkeydown={handleKeyDown}
		tabindex="0"
		bind:this={itemsMenu}
	>
		<div
			class="grid grid-cols-[2rem_2fr_2fr_1fr_2fr] bg-bg-1 text-sm font-medium shrink-0"
		>
			<div></div>
			{#each tableColumns as column}
				<button 
					class="px-2 py-2 truncate text-left flex items-center group cursor-pointer"
					onclick={() => handleColumnClick(column.sortKey as SortColumn)}
				>
					{column.title}
					<ArrowIcon class="h-4 transition-all duration-300 {$sortBy === column.sortKey ? 'text-fg-0' : 'text-fg-3/0 group-hover:text-fg-3'}" />
				</button>
			{/each}
		</div>
		<div class="flex-1 min-h-0 overflow-y-auto">
			{#each filesWithBack as file, fileIndex}
				<div
					bind:this={rowEls[fileIndex]}
					class={`grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center select-none text-sm box-border outline-none
					${fileIndex >= Math.min(selectedIndexStart, selectedIndexEnd) && fileIndex <= Math.max(selectedIndexStart, selectedIndexEnd)
						? "bg-primary/20 shadow-[inset_0_0_0_1px] shadow-primary"
						: "hover:bg-bg-2/50"}`}
					role="button"
					onclick={(e) => handleItemClick(fileIndex, e)}
					onkeydown={() => {}}
					tabindex="-1"
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
						{file.typeLong ?? ""}
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
			bind:this={searchBar}
			onkeydown={searchKeyDown}
		/>
	</div>
</div>
