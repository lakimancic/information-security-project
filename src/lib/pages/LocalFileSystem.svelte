<script lang="ts">
	import FileExplorer from '$lib/components/ui/file-explorer/FileExplorer.svelte';
	import type {
		LocalFile,
		ProgressFile
	} from '$lib/components/ui/file-explorer/utils';
	import KeyDialog from '$lib/components/ui/key-dialog/KeyDialog.svelte';
	import * as Select from '$lib/components/ui/select/index';
	import {
		blockCiphers,
		blockModes,
		type CryptoError,
		streamCiphers,
		type Key,
		padModes
	} from '$lib/types/crypto';
	import {
		EyeOffIcon,
		LockIcon,
		LockOpenIcon,
		ScanEyeIcon
	} from '@lucide/svelte';
	import { invoke } from '@tauri-apps/api/core';
	import { listen } from '@tauri-apps/api/event';
	import { onMount } from 'svelte';
	import { SvelteMap } from 'svelte/reactivity';
	import { notify } from '$lib/components/ui/notifications/store';

	let sourceFiles: LocalFile[] = $state([]);
	let sourceCwd: string = $state('');

	let destFiles: LocalFile[] = $state([]);
	let destCwd: string = $state('');

	let processFiles = $state<SvelteMap<string, ProgressFile>>(new SvelteMap());

	let algoStr = $state('');
	let modeStr = $state('');
	let padStr = $state('');

	let operation = $state<'dec' | 'enc'>('enc');
	let key = $state<Key | null>(null);
	let cachedKeys = $state<{ [algoMode: string]: Key }>({});

	let destLocked = $state(false);
	let sourceWatch = $state(false);

	let selectedFile: LocalFile | null = $state(null);

	let algo = $derived.by(() => {
		return (
			[...blockCiphers, ...streamCiphers].find((v) => v.value === algoStr) ??
			null
		);
	});
	let mode = $derived.by(() => {
		return blockModes.find((v) => v.value === modeStr) ?? null;
	});

	let processFilesArray = $derived.by(() => {
		return Array.from(processFiles.values());
	});

	const triggerContent = $derived(
		streamCiphers.find((c) => c.value === algoStr)?.label ??
			blockCiphers.find((c) => c.value === algoStr)?.label ??
			'Select Cipher'
	);

	const triggerContentMode = $derived(
		blockModes.find((m) => m.value === modeStr)?.label ?? 'Select Mode'
	);

	const triggerContentPad = $derived(
		padModes.find((m) => m.value === padStr)?.label ?? 'Select Padding'
	);

	const loadFiles = async (source: boolean, reset: boolean = false) => {
		await invoke('get_files', { source, reset })
			.then((res: any) => {
				if (source) {
					sourceFiles = res.files as LocalFile[];
					sourceCwd = res.pwd as string;
				} else {
					destFiles = res.files as LocalFile[];
					destCwd = res.pwd as string;
				}
			})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const changeDir = async (newDir: string, source: boolean) => {
		try {
			const res: boolean = await invoke('change_dir', { newDir, source });
			if (res) await loadFiles(source);

			return res;
		} catch (err) {
			return false;
		}
	};

	const goDirBack = async (source: boolean) => {
		try {
			const res: boolean = await invoke('go_dir_back', { source });
			if (res) await loadFiles(source);

			return res;
		} catch (err) {
			return false;
		}
	};

	const setAbsolutePath = async (newDir: string, source: boolean) => {
		try {
			const res: boolean = await invoke('set_current_dir', { newDir, source });
			if (res) await loadFiles(source);

			return res;
		} catch (err) {
			return false;
		}
	};

	const onFileAction = async (filename: string) => {
		if (operation === 'enc') {
			if (algo === null) {
				notify.warning('Algorithm is not selected', 3000);
				return;
			} else if (algoStr.startsWith('block:') && mode === null) {
				notify.warning('Block mode is not selected', 3000);
				return;
			}
		}

		if (key === null) {
			notify.warning('Key is not selected', 3000);
			return;
		}

		if (!destLocked) {
			notify.warning(
				`Before ${operation === 'enc' ? 'encryption' : 'decryption'} destination directory must be locked.`,
				3000
			);
			return;
		}

		let invoker;
		if (operation === 'enc') {
			invoker = invoke('encrypt_file', {
				request: {
					algorithm: algoStr,
					mode: mode ? modeStr : undefined,
					key: key.key,
					iv: key.iv,
					padding: padStr.length === 0 ? undefined : padStr
				},
				file: filename
			});
		} else {
			invoker = invoke('decrypt_file', {
				key: {
					key: key.key,
					iv: key.iv
				},
				file: filename
			});
		}

		invoker
			.then(() => {})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const stopFileEncryption = async (filename: string) => {
		const realFilename =
			operation === 'enc' ? filename.replace(/\.enc$/, '') : `${filename}.enc`;
		invoke('stop_processing', {
			filename: realFilename,
			encrypt: operation === 'enc'
		})
			.then(() => {
				processFiles.delete(realFilename);
			})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const onFileSystemWatch = async (locked: boolean): Promise<boolean> => {
		if (key === null) {
			notify.warning('Key is not selected', 3000);
			return false;
		}

		if (operation === 'enc') {
			if (algo === null) {
				notify.warning('Algorithm is not selected', 3000);
				return false;
			} else if (algoStr.startsWith('block:') && mode === null) {
				notify.warning('Block mode is not selected', 3000);
				return false;
			}
		}

		if (!destLocked) {
			notify.warning(
				`Before watching file system, destination directory must be locked.`,
				3000
			);
			return false;
		}

		if (!locked) {
			try {
				await invoke('stop_file_watching');
				return true;
			} catch (err: any) {
				return false;
			}
		} else {
			let mode =
				operation === 'enc'
					? {
							Encrypt: {
								algorithm: algoStr,
								mode: modeStr,
								key: key.key,
								iv: key.iv
							}
						}
					: {
							Decrypt: {
								key: key.key,
								iv: key.iv
							}
						};

			try {
				await invoke('start_file_watching', { mode });
				return true;
			} catch (err: any) {
				notify.error(err, 3000);
				return false;
			}
		}
	};

	const onKeySet = (newKey: Key) => {
		cachedKeys[algoStr + ':' + modeStr] = newKey;
	};

	const onAlgoSelect = (newAlgo: string) => {
		if (cachedKeys[newAlgo + ':' + modeStr])
			key = cachedKeys[newAlgo + ':' + modeStr];
		else key = null;
	};

	const onModeSelect = (newMode: string) => {
		if (cachedKeys[algoStr + ':' + newMode])
			key = cachedKeys[algoStr + ':' + newMode];
		else key = null;
	};

	const onDestLockChange = async () => {
		if (processFiles.size === 0 && !sourceWatch) return true;
		return false;
	};

	onMount(() => {
		loadFiles(true, true);
		loadFiles(false, true);

		const unlisteners: Array<() => void> = [];

		const setupListeners = async () => {
			unlisteners.push(
				await listen<ProgressFile>('crypto:start', (event) => {
					if (!processFiles.has(event.payload.filename)) {
						processFiles.set(event.payload.filename, {
							...event.payload,
							size: event.payload.total
						});
					}
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('crypto:done', (event) => {
					if (processFiles.has(event.payload.filename)) {
						processFiles.delete(event.payload.filename);
					}
					loadFiles(false);
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('crypto:progress', (event) => {
					if (processFiles.has(event.payload.filename)) {
						processFiles.set(event.payload.filename, {
							...event.payload,
							size: event.payload.total
						});
					}
				})
			);

			unlisteners.push(
				await listen<CryptoError>('crypto:error', (event) => {
					notify.error(event.payload.err, 3000);
					processFiles.delete(event.payload.filename);
				})
			);

			unlisteners.push(
				await listen('fsw:error', (event) => {
					notify.error(event.payload as string, 3000);
				})
			);
		};

		setupListeners();

		return () => {
			[...processFiles.values()].forEach((file) =>
				stopFileEncryption(file.filename)
			);
			unlisteners.forEach((fn) => fn());
		};
	});
</script>

<div class="flex flex-wrap items-center px-5 py-1">
	<p class="mr-3">Choose algorithm:</p>
	<Select.Root
		type="single"
		bind:value={algoStr}
		onValueChange={onAlgoSelect}
		disabled={operation === 'dec' || processFiles.size !== 0 || sourceWatch}
	>
		<Select.Trigger class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40">
			{triggerContent}
		</Select.Trigger>
		<Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
			<Select.Group>
				<Select.Label class="text-fg-3">Stream Ciphers</Select.Label>
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
	{#if algoStr.startsWith('block:')}
		<p class="mx-3">Choose mode:</p>
		<Select.Root
			type="single"
			bind:value={modeStr}
			onValueChange={onModeSelect}
			disabled={operation === 'dec' || processFiles.size !== 0 || sourceWatch}
		>
			<Select.Trigger class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40">
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
		<p class="mx-3">Choose padding:</p>
		<Select.Root
			type="single"
			bind:value={padStr}
			disabled={operation === 'dec' || processFiles.size !== 0 || sourceWatch}
		>
			<Select.Trigger class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40">
				{triggerContentPad}
			</Select.Trigger>
			<Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
				<Select.Group>
					<Select.Item
						value=""
						label="None"
						disabled={padStr === ''}
						class="hover:text-fg-0 hover:bg-bg-3/50"
					>
						None
					</Select.Item>
					{#each padModes as padMode}
						<Select.Item
							value={padMode.value}
							label={padMode.label}
							disabled={modeStr === padMode.value}
							class="hover:text-fg-0 hover:bg-bg-3/50"
						>
							{padMode.label}
						</Select.Item>
					{/each}
				</Select.Group>
			</Select.Content>
		</Select.Root>
	{/if}
	<p class="ml-auto">Key:</p>
	<p class="mx-2 {key !== null ? 'text-primary font-black' : 'text-fg-4'}">
		{key?.label ?? 'No key selected'}
	</p>
	<KeyDialog {algo} {mode} bind:outputKey={key} {onKeySet} {operation} />
	<p class="mr-2">Operation:</p>
	<div class="border-bg-4 flex gap-2 rounded-sm border p-1">
		<button
			class="border-2 {operation === 'dec'
				? 'border-bg-4'
				: processFiles.size > 0
					? 'border-bg-2'
					: 'border-primary'} cursor-pointer rounded-sm p-2"
			onclick={() => processFiles.size === 0 && (operation = 'enc')}
			>Encryption</button
		>
		<button
			class="border-2 {operation === 'enc'
				? 'border-bg-4'
				: processFiles.size > 0
					? 'border-bg-2'
					: 'border-primary'} cursor-pointer rounded-sm p-2"
			onclick={() => processFiles.size === 0 && (operation = 'dec')}
			>Decryption</button
		>
	</div>
	<button
		class="bg-primary hover:bg-primary/60 disabled:bg-bg-5 disabled:text-bg-1 dark:disabled:text-fg-3
        text-bg-0 dark:text-fg-0 ml-3 cursor-pointer rounded-sm px-4 py-3 transition-colors duration-300"
		disabled={key === null ||
			selectedFile === null ||
			!destLocked ||
			sourceWatch ||
			processFiles.size !== 0}
		onclick={() => selectedFile && onFileAction(selectedFile.filename)}
		>{operation === 'enc' ? 'Encrypt' : 'Decrypt'}</button
	>
</div>
<div class="flex min-h-0 flex-1">
	<div class="flex min-h-0 flex-1 flex-col overflow-hidden p-4">
		<FileExplorer
			class="h-full"
			bind:pwd={sourceCwd}
			files={sourceFiles}
			label={`Source Directory${sourceWatch ? ' (Watching)' : ''}`}
			bind:locked={sourceWatch}
			bind:selectedFile
			lockIcon={ScanEyeIcon}
			unlockIcon={EyeOffIcon}
			onGoBack={async () => await goDirBack(true)}
			onChangeDir={async (dir) => await changeDir(dir, true)}
			{onFileAction}
			onStopProcessingFile={() => {}}
			onSetAbsolutePath={async (newDir) => await setAbsolutePath(newDir, true)}
			onLockChange={onFileSystemWatch}
			onRefresh={() => loadFiles(true)}
			constFilter={operation === 'dec' ? /^.*\.enc$/ : undefined}
			processingFiles={[]}
			pendingFiles={[]}
			onAcceptRejectFile={() => {}}
		/>
	</div>
	<div class="flex min-h-0 flex-1 flex-col overflow-hidden p-4">
		<FileExplorer
			class="h-full"
			pwd={destCwd}
			files={destFiles}
			label={`Destination Directory${destLocked ? ' (Locked)' : ''}`}
			bind:locked={destLocked}
			selectedFile={null}
			lockIcon={LockIcon}
			unlockIcon={LockOpenIcon}
			onGoBack={() => goDirBack(false)}
			onChangeDir={(dir) => changeDir(dir, false)}
			onFileAction={() => {}}
			onStopProcessingFile={stopFileEncryption}
			onSetAbsolutePath={async (newDir) => await setAbsolutePath(newDir, false)}
			onLockChange={onDestLockChange}
			onRefresh={() => loadFiles(false)}
			constFilter={operation === 'enc' ? /^.*\.enc$/ : undefined}
			processingFiles={processFilesArray}
			pendingFiles={[]}
			onAcceptRejectFile={() => {}}
		/>
	</div>
</div>
