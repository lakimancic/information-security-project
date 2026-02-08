<script lang="ts">
	import FileExplorer from '$lib/components/ui/file-explorer/FileExplorer.svelte';
	import type { LocalFile } from '$lib/components/ui/file-explorer/utils';
	import { notify } from '$lib/components/ui/notifications/store';
	import * as Select from '$lib/components/ui/select/index';
	import {
		blockCiphers,
		blockModes,
		streamCiphers,
		type ShortKey
	} from '$lib/types/crypto';
	import { sizesToAlgorithm } from '$lib/utils/crypto';
	import {
		FolderOpenIcon,
		KeyIcon,
		LockIcon,
		LockOpenIcon,
		SaveAllIcon,
		Trash2Icon
	} from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { Dialog, Separator } from 'bits-ui';
	import { onMount } from 'svelte';

	let files: LocalFile[] = $state([]);
	let pwd: string = $state('');
	let filesLocked = $state(false);

	let saveFile = $state('');

	let algoStr = $state('');
	let modeStr = $state('');

	let errorMsg = $state('');
	let dialogOpen = $state(false);

	let genName = $state('');
	let genPass = $state('');
	let genPassRep = $state('');

	let keys: ShortKey[] = $state([]);

	let selectedFile: LocalFile | null = $state(null);

	const triggerContent = $derived(
		streamCiphers.find((c) => c.value === algoStr)?.label ??
			blockCiphers.find((c) => c.value === algoStr)?.label ??
			'Select Cipher'
	);

	const triggerContentMode = $derived(
		blockModes.find((m) => m.value === modeStr)?.label ?? 'Select Mode'
	);

	const loadFiles = async (reset: boolean = false) => {
		await invoke('get_files', { source: true, reset }).then((res: any) => {
			files = res.files as LocalFile[];
			pwd = res.pwd as string;
		});
	};

	const listKeys = async () => {
		await invoke<ShortKey[]>('list_keys').then((res) => {
			keys = res;
		});
	};

	const handleGenerateKey = async () => {
		if (algoStr === '') {
			errorMsg = 'Algorithm is not selected.';
			return;
		}

		if (algoStr.startsWith('block:') && modeStr === '') {
			errorMsg = 'Block mode is not selected.';
			return;
		}

		if (genName.length < 3) {
			errorMsg = 'Key name must be at least 3 characters long';
			return;
		}

		if (genPass.length < 3) {
			errorMsg = 'Password is too short (min 3 characters)';
			return;
		}

		if (genPass !== genPassRep) {
			errorMsg = "Passwords doesn't match";
			return;
		}

		invoke('generate_new_key', {
			algorithm: algoStr,
			mode: modeStr,
			name: genName,
			password: genPass
		})
			.then(() => {
				listKeys();
				dialogOpen = false;
			})
			.catch((err: any) => {
				errorMsg = err.message;
			});
	};

	const loadKeys = async () => {
		if (!selectedFile) return;

		await invoke('load_keys', { filename: selectedFile.filename })
			.then(() => {
				listKeys();
			})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const removeKey = async (name: string) => {
		await invoke('remove_key', { name }).then(() => {
			listKeys();
		});
	};

	const saveKeys = async () => {
		await invoke('save_keys', { filename: saveFile })
			.then(() => {
				notify.success(`Keys are saved to file ${saveFile}.keys`, 3000);
				saveFile = '';
				loadFiles();
			})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const changeDir = async (newDir: string) => {
		try {
			const res: boolean = await invoke('change_dir', { newDir, source: true });
			if (res) await loadFiles(true);

			return res;
		} catch (err) {
			return false;
		}
	};

	const goDirBack = async () => {
		try {
			const res: boolean = await invoke('go_dir_back', { source: true });
			if (res) await loadFiles(true);

			return res;
		} catch (err) {
			return false;
		}
	};

	const setAbsolutePath = async (newDir: string) => {
		try {
			const res: boolean = await invoke('set_current_dir', {
				newDir,
				source: true
			});
			if (res) await loadFiles(true);

			return res;
		} catch (err) {
			return false;
		}
	};

	onMount(() => {
		loadFiles(true);
		listKeys();
	});
</script>

<div class="grid h-full min-h-0 grid-cols-2">
	<div class="flex min-h-0 flex-col overflow-hidden p-4">
		<FileExplorer
			class="h-full"
			bind:pwd
			{files}
			label="Look for Keys in directory"
			locked={filesLocked}
			bind:selectedFile
			lockIcon={LockIcon}
			unlockIcon={LockOpenIcon}
			onGoBack={async () => await goDirBack()}
			onChangeDir={async (dir) => await changeDir(dir)}
			onFileAction={() => {}}
			onStopProcessingFile={() => {}}
			onSetAbsolutePath={async (newDir) => await setAbsolutePath(newDir)}
			onLockChange={async () => true}
			onRefresh={() => loadFiles(true)}
			constFilter={/^.*\.keys$/}
			processingFiles={[]}
			pendingFiles={[]}
			onAcceptRejectFile={() => {}}
		/>
	</div>
	<div class="flex min-h-0 flex-col gap-2 p-4">
		<div class="flex justify-around gap-5 py-5">
			<button
				class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 disabled:bg-bg-5 disabled:text-bg-1 dark:disabled:text-fg-3 flex cursor-pointer items-center
                gap-3 rounded-md px-4 py-3 text-xl transition-all duration-200"
				disabled={selectedFile === null}
				onclick={() => loadKeys()}
			>
				Load Keys <FolderOpenIcon />
			</button>
			<Dialog.Root bind:open={dialogOpen}>
				<Dialog.Trigger
					class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 flex cursor-pointer items-center gap-3 rounded-md px-4
                    py-3 text-xl transition-all duration-200"
				>
					Generate New <KeyIcon />
				</Dialog.Trigger>
				<Dialog.Portal>
					<Dialog.Overlay
						class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 bg-bg-0/80 fixed inset-0 z-50"
					/>
					<Dialog.Content
						class="bg-bg-1 text-fg-0 border-bg-4 data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 
                        data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] 
                        left-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] rounded-md border p-5 outline-hidden sm:max-w-[490px] md:w-full"
					>
						<Dialog.Title
							class="flex w-full items-center justify-center text-xl font-semibold tracking-tight"
						>
							Generate New Key
						</Dialog.Title>
						<Separator.Root class="bg-bg-4 -mx-5 mt-5 mb-6 block h-px" />
						<Dialog.Description class="text-fg-3 mt-2 mb-6 text-sm">
							Generate new key for specific algorithms and block modes. All keys
							are generated by secure pseudo random generator.
						</Dialog.Description>
						<div class="flex items-center gap-4 py-2">
							<p>Choose algorithm:</p>
							<Select.Root type="single" bind:value={algoStr}>
								<Select.Trigger
									class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40"
								>
									{triggerContent}
								</Select.Trigger>
								<Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
									<Select.Group>
										<Select.Label class="text-fg-3">Stream Ciphers</Select.Label
										>
										{#each streamCiphers as cipher}
											<Select.Item
												value={cipher.value}
												label={cipher.label}
												disabled={algoStr === cipher.value}
												class="hover:text-fg-0 hover:bg-bg-3/50"
											>
												{cipher.label}
											</Select.Item>
										{/each}
									</Select.Group>
									<Select.Group>
										<Select.Label class="text-fg-3">Block Ciphers</Select.Label>
										{#each blockCiphers as cipher}
											<Select.Item
												value={cipher.value}
												label={cipher.label}
												disabled={algoStr === cipher.value}
												class="hover:text-fg-0 hover:bg-bg-3/50"
											>
												{cipher.label}
											</Select.Item>
										{/each}
									</Select.Group>
								</Select.Content>
							</Select.Root>
						</div>
						{#if algoStr.startsWith('block:')}
							<div class="flex items-center gap-4 py-2">
								<p>Choose block mode:</p>
								<Select.Root type="single" bind:value={modeStr}>
									<Select.Trigger
										class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40"
									>
										{triggerContentMode}
									</Select.Trigger>
									<Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
										<Select.Group>
											{#each blockModes as blockMode}
												<Select.Item
													value={blockMode.value}
													label={blockMode.label}
													disabled={modeStr === blockMode.value}
													class="hover:text-fg-0 hover:bg-bg-3/50"
												>
													{blockMode.label}
												</Select.Item>
											{/each}
										</Select.Group>
									</Select.Content>
								</Select.Root>
							</div>
						{/if}
						<div class="flex items-center gap-4 py-2">
							<p>Enter new key name:</p>
							<input
								type="text"
								name="key_name"
								id="keyName"
								placeholder="key_name"
								class="border-bg-5 placeholder:text-fg-0/40 min-w-40 rounded-md border px-3 py-1.5 outline-none"
								bind:value={genName}
							/>
						</div>
						<div class="flex items-center gap-4 py-2">
							<p>Enter key password:</p>
							<input
								type="password"
								name="key_password"
								id="keyPassword"
								placeholder="key_password"
								class="border-bg-5 placeholder:text-fg-0/40 min-w-40 rounded-md border px-3 py-1.5 outline-none"
								bind:value={genPass}
							/>
						</div>
						<div class="flex items-center gap-4 py-2">
							<p>Confirm key password:</p>
							<input
								type="password"
								name="confirm_password"
								id="confirmPassword"
								placeholder="confirm_password"
								class="border-bg-5 placeholder:text-fg-0/40 min-w-40 rounded-md border px-3 py-1.5 outline-none"
								bind:value={genPassRep}
							/>
						</div>
						<p class="text-error text-center">{errorMsg}</p>
						<div class="mt-5 flex w-full justify-end">
							<button
								class="bg-primary hover:bg-primary/70 cursor-pointer rounded-md px-4 py-3 font-semibold transition-colors duration-300"
								onclick={handleGenerateKey}>Generate Key</button
							>
						</div>
					</Dialog.Content>
				</Dialog.Portal>
			</Dialog.Root>
			<button
				class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 disabled:bg-bg-5 disabled:text-bg-1 dark:disabled:text-fg-3 flex cursor-pointer items-center
                gap-3 rounded-md px-4 py-3 text-xl transition-all duration-200"
				disabled={saveFile.length === 0}
				onclick={saveKeys}
			>
				Save Keys <SaveAllIcon />
			</button>
		</div>
		<div class="flex items-center py-3">
			<p class="mr-5">Save Keys file name:</p>
			<input
				type="text"
				class="border-bg-5 data-[placeholder]:text-fg-3 w-60 rounded-md border px-3 py-1 outline-none"
				placeholder="data"
				bind:value={saveFile}
			/>
			<p>.keys</p>
		</div>
		<div
			class="border-fg-5/50 relative m-7 flex min-h-0 flex-1 flex-col border font-semibold"
		>
			<div
				class="bg-bg-0 sticky top-0 z-10 grid w-full grid-cols-[3fr_2fr_4fr]"
			>
				{#each ['Key name', 'Key size', 'Possible Algorithms'] as colName, colIndex}
					<p
						class="border-b px-3 py-2 {colIndex < 2 &&
							'border-r'} border-fg-5/50 bg-fg-3/10"
					>
						{colName}
					</p>
				{/each}
			</div>
			<div
				class="relative z-8 grid min-h-0 auto-rows-max grid-cols-[3fr_2fr_4fr] content-start items-start overflow-auto"
			>
				{#each keys as key}
					<p class="border-fg-5/50 truncate border-r border-b px-3 py-2">
						{key.name}
					</p>
					<p class="border-fg-5/50 truncate border-r border-b px-3 py-2">
						{key.keySize}{key.ivSize > 0 ? `+${key.ivSize}` : ''} B
					</p>
					<p
						class="text-primary border-fg-5/50 relative flex items-center truncate border-b px-3 py-2"
					>
						{sizesToAlgorithm(key.keySize, key.ivSize).join(', ') ?? '...'}
						<button
							class="text-error hover:text-error/80 absolute right-5 cursor-pointer"
							onclick={() => removeKey(key.name)}
						>
							<Trash2Icon />
						</button>
					</p>
				{/each}
			</div>
		</div>
	</div>
</div>
