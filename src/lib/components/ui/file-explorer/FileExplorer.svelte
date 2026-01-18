<script lang="ts">
	import type { LocalFile, PendingFile, ProgressFile } from '$lib/components/ui/file-explorer/utils';
	import { sizeToString, typeToIcon } from '$lib/components/ui/file-explorer/utils';
	import { writable } from 'svelte/store';
	import { ArrowDown, ArrowUp, CheckIcon, RotateCwIcon, XIcon, type IconProps } from '@lucide/svelte';
	import type { Component } from 'svelte';

	let {
		"class": className,
		files = [],
		processingFiles,
		pendingFiles,
		pwd = $bindable(''),
		label,
		locked = $bindable(false),
		selectedIndex = $bindable(-1),
		lockIcon : LockIcon,
		unlockIcon : UnlockIcon,
		constFilter,

		onGoBack,
		onChangeDir,
		onFileAction,
		onStopProcessingFile,
		onSetAbsolutePath,
		onLockChange,
		onRefresh,
		onAcceptRejectFile,
	} : {
		"class": string;
		files: LocalFile[];
		pendingFiles: PendingFile[];
		processingFiles: ProgressFile[];
		pwd: string;
		label: string;
		locked: boolean;
		selectedIndex: number;
		lockIcon: Component<IconProps, {}>;
		unlockIcon: Component<IconProps, {}>;
		constFilter?: RegExp;

		onGoBack: () => Promise<boolean>;
		onChangeDir: (newDir: string) => Promise<boolean>;
		onFileAction: (filename: string) => void;
		onStopProcessingFile: (filename: string) => void;
		onSetAbsolutePath: (path: string) => Promise<boolean>;
		onLockChange: (value: boolean) => Promise<boolean>;
		onRefresh: () => void;
		onAcceptRejectFile: (sockAddr: string, accept: boolean) => void;
	} = $props();

	const searchText = writable('');
	let searchFilter = $state(new RegExp(""));
	let oldPwd = $state('');

	const indexStack = $state<number[]>([]);

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

	// let selectedIndex = $state(-1);
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
			if (constFilter && file.fileType !== "folder") {
				return constFilter.test(file.filename) && searchFilter.test(file.filename);
			}
			return searchFilter.test(file.filename);
		});
		
		return [
			{ filename: "..", lastModified: "", fileType: "back", size: 0 },
			...filesCopy
		];
	});

	const clearSearch = () => {
		searchFilter = new RegExp('');
		searchText.update(() => '');
	};

	const pwdFocusIn = () => {
		oldPwd = pwd;
	};

	const pwdFocusOut = () => {
		pwd = oldPwd;
	};

	const handleGoBack = async () => {
		if (locked) return;

		if (await onGoBack()) {
			clearSearch();
			selectedIndex = indexStack.pop() ?? -1;
		}
	};

	const handleChangeDir = async (newDir: string, index: number) => {
		if (locked) return;

		if (await onChangeDir(newDir)) {
			clearSearch();
			indexStack.push(index);
			selectedIndex = -1;
		}
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
			selectedIndex--;
		} else if (e.key === 'ArrowDown') {
			selectedIndex++;
		} else if (e.key === 'ArrowLeft') {
			handleGoBack();
		} else if (e.key === 'ArrowRight') {
			const selectedFile = filesWithBack[selectedIndex];
			if (selectedFile.fileType === "folder") {
				handleChangeDir(selectedFile.filename, selectedIndex);
				return;
			}
			onFileAction(selectedFile.filename);
		} else if (e.key === '/') {
			searchBar.focus();
		} else if (e.key === '?') {
			pwdBar.focus();
		}
		selectedIndex = Math.min(Math.max(selectedIndex, 1), filesWithBack.length-1);
	};

	const handleItemClick = (fileIndex: number, e: MouseEvent) => {
		if (fileIndex === 0) return;

		selectedIndex = fileIndex;
	};

	const handleItemDoubleClick = (file: LocalFile, index: number) => {
		if (file.fileType === "back") {
			handleGoBack();
		} else if (file.fileType == "folder") {
			handleChangeDir(file.filename, index);
		} else {
			onFileAction(file.filename);
		}
	};

	const escapeItemsMenu = (e: KeyboardEvent) => {
		if (e.key === 'Escape') {
			e.preventDefault();
			itemsMenu.focus();
		}
	};

	const pwdKeyDown = async (e: KeyboardEvent) => {
		if (e.key === 'Enter') {
			clearSearch();
			if (await onSetAbsolutePath(pwd)) {
				oldPwd = pwd;
				clearSearch();
			}
		}

		escapeItemsMenu(e);
	};

	const searchKeyDown = (e: KeyboardEvent) => {
		escapeItemsMenu(e);

		if (e.key === 'Enter') {
			selectedIndex = -1;
			searchFilter = new RegExp($searchText);
		}
	};

	const handleLockClick = async () => {
		if (await onLockChange(!locked)) {
			locked = !locked;
		}
	};

	const handleRefreshClick = () => {
		if (locked) return;

		onRefresh();
	};

	$effect(() => {
		const i = selectedIndex;
		if (i < 0) return;

		const row = rowEls[i === 1 ? 0 : i];
		if (!row) return;
		
		row.scrollIntoView({
			block: 'nearest',
			behavior: 'auto'
		});
	});
</script>

<div class="flex flex-col {className} box-border">
	<p class="p-2 text-center text-xl text-fg-2">{label}</p>
	<div class="flex bg-bg-2 p-2 text-sm text-primary font-bold gap-2">
		<input
			type="text"
			bind:value={pwd}
			class="w-full border-none disabled:text-primary/50 outline-none"
			onfocusin={pwdFocusIn}
			onfocusout={pwdFocusOut}
			bind:this={pwdBar}
			onkeydown={pwdKeyDown}
			disabled={locked}
		/>
		<RotateCwIcon 
			class="cursor-pointer text-fg-3 hover:text-fg-0 transition-colors duration-300 data-[disabled=true]:text-fg-3/60" 
			onclick={handleRefreshClick}
			data-disabled={locked}
		/>
		{#if locked}
			<UnlockIcon class="cursor-pointer text-fg-2 hover:text-primary transition-colors duration-300" onclick={handleLockClick} />
		{:else}
			<LockIcon class="cursor-pointer text-fg-2 hover:text-primary transition-colors duration-300" onclick={handleLockClick} />
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
			{#each pendingFiles as file, fileIndex}
				<div
					bind:this={rowEls[fileIndex]}
					class={`grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center select-none text-sm box-border outline-none bg-warning/10 hover:bg-warning/15 relative`}
					role="button"
					onkeydown={() => {}}
					tabindex="-1"
				>
					<div class="py-2 pl-2">
						<img
							src={typeToIcon("")}
							class="h-6"
							alt="file-icon"
						/>
					</div>

					<div class="px-2 py-2 truncate">
						{file.filename}
					</div>

					<div class="px-4 py-2 truncate w-full">
						Sender: {file.sockAddr}
					</div>

					<div class="px-4 py-2 text-left">
						{file.size ? sizeToString(file.size) : ""}
					</div>

					<div class="px-4 py-2 truncate">Pending File</div>
					<div class="flex absolute right-5">
						<button
						class="text-success cursor-pointer hover:bg-success/30 rounded-full p-1"
						onclick={() => onAcceptRejectFile(file.sockAddr, true)}
						>
							<CheckIcon />
						</button>
						<button
						class="text-error cursor-pointer hover:bg-error/30 rounded-full p-1"
						onclick={() => onAcceptRejectFile(file.sockAddr, false)}
						>
							<XIcon />
						</button>
					</div>
				</div>
			{/each}
			{#each processingFiles as file, fileIndex}
				<div
					bind:this={rowEls[fileIndex]}
					class={`grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center select-none text-sm box-border outline-none bg-primary/10 hover:bg-primary/15 relative`}
					role="button"
					onkeydown={() => {}}
					tabindex="-1"
				>
					<div class="py-2 pl-2">
						<img
							src={typeToIcon("")}
							class="h-6"
							alt="file-icon"
						/>
					</div>

					<div class="px-2 py-2 truncate">
						{file.filename}
					</div>

					<div class="px-4 py-2 truncate w-full">
						<div class="w-full border border-fg-0/20 h-6 rounded-md overflow-hidden">
							<div 
								class="h-full bg-success"
								style="width: {(100 * file.processed / file.total) || 0}%"
							></div>
						</div>
					</div>

					<div class="px-4 py-2 text-left">
						{file.size ? sizeToString(file.size) : ""}
					</div>

					<div class="px-4 py-2 truncate">Encrypted File</div>
					<button
						class="absolute right-5 text-error cursor-pointer hover:bg-error/30 rounded-full p-1"
						onclick={() => onStopProcessingFile(file.filename)}
					>
						<XIcon />
					</button>
				</div>
			{/each}
			{#each filesWithBack as file, fileIndex}
				<div
					bind:this={rowEls[fileIndex]}
					class={`grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center select-none text-sm box-border outline-none
					${fileIndex == selectedIndex
						? `shadow-[inset_0_0_0_1px] ${locked ? "shadow-fg-5 bg-fg-5/20" : "shadow-primary bg-primary/20"}`
						: "hover:bg-bg-2/50"}`}
					role="button"
					onclick={(e) => handleItemClick(fileIndex, e)}
					ondblclick={() => handleItemDoubleClick(file, fileIndex)}
					onkeydown={() => {}}
					tabindex="-1"
				>
					<div class="py-2 pl-2">
						<img
							src={typeToIcon(file.fileType ?? "")}
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
			class="w-full p-2 border border-bg-4 rounded outline-none disabled:text-fg-2/60"
			bind:this={searchBar}
			onkeydown={searchKeyDown}
			disabled={locked}
		/>
	</div>
</div>
