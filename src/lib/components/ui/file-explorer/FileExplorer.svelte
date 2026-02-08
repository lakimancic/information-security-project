<script lang="ts">
	import type {
		LocalFile,
		PendingFile,
		ProgressFile
	} from '$lib/components/ui/file-explorer/utils';
	import {
		sizeToString,
		typeToIcon
	} from '$lib/components/ui/file-explorer/utils';
	import { writable } from 'svelte/store';
	import {
		ArrowDown,
		ArrowUp,
		CheckIcon,
		RotateCwIcon,
		XIcon,
		type IconProps
	} from '@lucide/svelte';
	import type { Component } from 'svelte';

	let {
		class: className,
		files = [],
		processingFiles,
		pendingFiles,
		pwd = $bindable(''),
		label,
		locked = $bindable(false),
		selectedFile = $bindable(null),
		lockIcon: LockIcon,
		unlockIcon: UnlockIcon,
		constFilter,

		onGoBack,
		onChangeDir,
		onFileAction,
		onStopProcessingFile,
		onSetAbsolutePath,
		onLockChange,
		onRefresh,
		onAcceptRejectFile
	}: {
		class: string;
		files: LocalFile[];
		pendingFiles: PendingFile[];
		processingFiles: ProgressFile[];
		pwd: string;
		label: string;
		locked: boolean;
		selectedFile: LocalFile | null;
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
	let searchFilter = $state(new RegExp(''));
	let oldPwd = $state('');

	const indexStack = $state<number[]>([]);

	let searchBar: HTMLInputElement;
	let pwdBar: HTMLInputElement;
	let itemsMenu: HTMLDivElement;

	let selectedIndex = $state(-1);

	type SortColumn = 'name' | 'date' | 'size' | 'type';
	let sortBy = writable<SortColumn>('name');
	let sortDirection = writable<'asc' | 'desc'>('asc');

	const ArrowIcon = $derived($sortDirection === 'asc' ? ArrowUp : ArrowDown);

	const tableColumns = [
		{ title: 'Filename', sortKey: 'name' },
		{ title: 'Last Modified', sortKey: 'date' },
		{ title: 'Size', sortKey: 'size' },
		{ title: 'Type', sortKey: 'type' }
	];

	let rowEls = $state<{ [key: number]: HTMLDivElement | null }>({});

	$effect(() => {
		if (selectedIndex !== -1 && selectedIndex < filesWithBack.length) {
			selectedFile = filesWithBack[selectedIndex];
		} else {
			selectedFile = null;
		}
	});

	const filesWithBack = $derived.by(() => {
		const filesCopy = files
			.toSorted((a, b) => {
				let res = 0;
				if ($sortBy === 'size') res = (a.size ?? 0) - (b.size ?? 0);
				else if ($sortBy === 'date')
					res =
						new Date(a.lastModified).getTime() -
						new Date(b.lastModified).getTime();
				else if ($sortBy === 'name') res = a.filename.localeCompare(b.filename);
				else res = a.typeLong?.localeCompare(b.typeLong ?? '') ?? 0;

				return res * ($sortDirection === 'asc' ? 1 : -1);
			})
			.filter((file) => {
				if (constFilter && file.fileType !== 'folder') {
					return (
						constFilter.test(file.filename) && searchFilter.test(file.filename)
					);
				}
				return searchFilter.test(file.filename);
			});

		return [
			{ filename: '..', lastModified: '', fileType: 'back', size: 0 },
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
			sortDirection.update((old) => (old === 'asc' ? 'desc' : 'asc'));
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
			if (selectedFile.fileType === 'folder') {
				handleChangeDir(selectedFile.filename, selectedIndex);
				return;
			}
			onFileAction(selectedFile.filename);
		} else if (e.key === '/') {
			searchBar.focus();
		} else if (e.key === '?') {
			pwdBar.focus();
		}
		selectedIndex = Math.min(
			Math.max(selectedIndex, 1),
			filesWithBack.length - 1
		);
	};

	const handleItemClick = (fileIndex: number, e: MouseEvent) => {
		if (fileIndex === 0) return;

		selectedIndex = fileIndex;
	};

	const handleItemDoubleClick = (file: LocalFile, index: number) => {
		if (file.fileType === 'back') {
			handleGoBack();
		} else if (file.fileType == 'folder') {
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
	<p class="text-fg-2 p-2 text-center text-xl">{label}</p>
	<div class="bg-bg-2 text-primary flex gap-2 p-2 text-sm font-bold">
		<input
			type="text"
			bind:value={pwd}
			class="disabled:text-primary/50 w-full border-none outline-none"
			onfocusin={pwdFocusIn}
			onfocusout={pwdFocusOut}
			bind:this={pwdBar}
			onkeydown={pwdKeyDown}
			disabled={locked}
		/>
		<RotateCwIcon
			class="text-fg-3 hover:text-fg-0 data-[disabled=true]:text-fg-3/60 cursor-pointer transition-colors duration-300"
			onclick={handleRefreshClick}
			data-disabled={locked}
		/>
		{#if locked}
			<UnlockIcon
				class="text-fg-2 hover:text-primary cursor-pointer transition-colors duration-300"
				onclick={handleLockClick}
			/>
		{:else}
			<LockIcon
				class="text-fg-2 hover:text-primary cursor-pointer transition-colors duration-300"
				onclick={handleLockClick}
			/>
		{/if}
	</div>

	<div
		class="flex min-h-0 w-full flex-1 flex-col text-left outline-none"
		role="menu"
		onkeydown={handleKeyDown}
		tabindex="0"
		bind:this={itemsMenu}
	>
		<div
			class="bg-bg-1 grid shrink-0 grid-cols-[2rem_2fr_2fr_1fr_2fr] text-sm font-medium"
		>
			<div></div>
			{#each tableColumns as column}
				<button
					class="group flex cursor-pointer items-center truncate px-2 py-2 text-left"
					onclick={() => handleColumnClick(column.sortKey as SortColumn)}
				>
					{column.title}
					<ArrowIcon
						class="h-4 transition-all duration-300 {$sortBy === column.sortKey
							? 'text-fg-0'
							: 'text-fg-3/0 group-hover:text-fg-3'}"
					/>
				</button>
			{/each}
		</div>
		<div class="min-h-0 flex-1 overflow-y-auto">
			{#each pendingFiles as file, fileIndex}
				<div
					bind:this={rowEls[fileIndex]}
					class={`bg-warning/10 hover:bg-warning/15 relative box-border grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center text-sm outline-none select-none`}
					role="button"
					onkeydown={() => {}}
					tabindex="-1"
				>
					<div class="py-2 pl-2">
						<img src={typeToIcon('')} class="h-6" alt="file-icon" />
					</div>

					<div class="truncate px-2 py-2">
						{file.filename}
					</div>

					<div class="w-full truncate px-4 py-2">
						Sender: {file.sockAddr}
					</div>

					<div class="px-4 py-2 text-left">
						{file.size ? sizeToString(file.size) : ''}
					</div>

					<div class="truncate px-4 py-2">Pending File</div>
					<div class="absolute right-5 flex">
						<button
							class="text-success hover:bg-success/30 cursor-pointer rounded-full p-1"
							onclick={() => onAcceptRejectFile(file.sockAddr, true)}
						>
							<CheckIcon />
						</button>
						<button
							class="text-error hover:bg-error/30 cursor-pointer rounded-full p-1"
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
					class={`bg-primary/10 hover:bg-primary/15 relative box-border grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center text-sm outline-none select-none`}
					role="button"
					onkeydown={() => {}}
					tabindex="-1"
				>
					<div class="py-2 pl-2">
						<img src={typeToIcon('')} class="h-6" alt="file-icon" />
					</div>

					<div class="truncate px-2 py-2">
						{file.filename}
					</div>

					<div class="w-full truncate px-4 py-2">
						<div
							class="border-fg-0/20 h-6 w-full overflow-hidden rounded-md border"
						>
							<div
								class="bg-success h-full"
								style="width: {(100 * file.processed) / file.total || 0}%"
							></div>
						</div>
					</div>

					<div class="px-4 py-2 text-left">
						{file.size ? sizeToString(file.size) : ''}
					</div>

					<div class="truncate px-4 py-2">Encrypted File</div>
					<button
						class="text-error hover:bg-error/30 absolute right-5 cursor-pointer rounded-full p-1"
						onclick={() => onStopProcessingFile(file.filename)}
					>
						<XIcon />
					</button>
				</div>
			{/each}
			{#each filesWithBack as file, fileIndex}
				<div
					bind:this={rowEls[fileIndex]}
					class={`box-border grid grid-cols-[auto_2fr_2fr_1fr_2fr] items-center text-sm outline-none select-none
					${
						fileIndex == selectedIndex
							? `shadow-[inset_0_0_0_1px] ${locked ? 'shadow-fg-5 bg-fg-5/20' : 'shadow-primary bg-primary/20'}`
							: 'hover:bg-bg-2/50'
					}`}
					role="button"
					onclick={(e) => handleItemClick(fileIndex, e)}
					ondblclick={() => handleItemDoubleClick(file, fileIndex)}
					onkeydown={() => {}}
					tabindex="-1"
				>
					<div class="py-2 pl-2">
						<img
							src={typeToIcon(file.fileType ?? '')}
							class="h-6"
							alt="file-icon"
						/>
					</div>

					<div class="truncate px-2 py-2">
						{file.filename}
					</div>

					<div class="truncate px-4 py-2">
						{file.lastModified}
					</div>

					<div class="px-4 py-2 text-left">
						{file.size ? sizeToString(file.size) : ''}
					</div>

					<div class="truncate px-4 py-2">
						{file.typeLong ?? ''}
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
			class="border-bg-4 disabled:text-fg-2/60 w-full rounded border p-2 outline-none"
			bind:this={searchBar}
			onkeydown={searchKeyDown}
			disabled={locked}
		/>
	</div>
</div>
