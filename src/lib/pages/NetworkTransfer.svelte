<script lang="ts">
	import FileExplorer from '$lib/components/ui/file-explorer/FileExplorer.svelte';
	import {
		HardDriveUploadIcon,
		LockIcon,
		LockOpenIcon,
		XCircleIcon
	} from '@lucide/svelte';
	import * as Select from '$lib/components/ui/select/index';
	import {
		blockCiphers,
		blockModes,
		type CryptoError,
		hashModes,
		streamCiphers,
		type Key
	} from '$lib/types/crypto';
	import KeyDialog from '$lib/components/ui/key-dialog/KeyDialog.svelte';
	import type {
		LocalFile,
		PendingFile,
		ProgressFile
	} from '$lib/components/ui/file-explorer/utils';
	import { SvelteMap } from 'svelte/reactivity';
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';
	import { listen } from '@tauri-apps/api/event';
	import { notify } from '$lib/components/ui/notifications/store';

	let sendFiles: LocalFile[] = $state([]);
	let sendCwd: string = $state('');

	let recvFiles: LocalFile[] = $state([]);
	let recvCwd: string = $state('');

	let recvQueue: PendingFile[] = $state([]);

	let sendingFiles = $state<SvelteMap<string, ProgressFile>>(new SvelteMap());
	let receivingFiles = $state<SvelteMap<string, ProgressFile>>(new SvelteMap());

	let algoStr = $state('');
	let modeStr = $state('');
	let hashStr = $state('');

	let sendIp = $state('');
	let sendPort = $state('');

	let recvPort = $state('');
	let recvKeyPort = $state('');

	let key = $state<Key | null>(null);
	let cachedKeys = $state<{ [algoMode: string]: Key }>({});
	let networkKeys = $state<SvelteMap<string, number[]>>(new SvelteMap());

	let recvLocked = $state(false);
	let sendLocked = $state(false);

	let selectedFile: LocalFile | null = $state(null);

	let fileListening = $state(false);
	let keyListening = $state(false);

	let algo = $derived.by(() => {
		return (
			[...blockCiphers, ...streamCiphers].find((v) => v.value === algoStr) ??
			null
		);
	});
	let mode = $derived.by(() => {
		return blockModes.find((v) => v.value === modeStr) ?? null;
	});

	const sendingFilesArray = $derived.by(() => {
		return Array.from(sendingFiles.values());
	});

	const receivingFilesArray = $derived.by(() => {
		return Array.from(receivingFiles.values());
	});

	const triggerContent = $derived(
		streamCiphers.find((c) => c.value === algoStr)?.label ??
			blockCiphers.find((c) => c.value === algoStr)?.label ??
			'Select Cipher'
	);

	const triggerContentMode = $derived(
		blockModes.find((m) => m.value === modeStr)?.label ?? 'Select Mode'
	);

	const triggerContentHash = $derived(
		hashModes.find((h) => h.value === hashStr)?.label ?? 'Select Hash'
	);

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

	const loadFiles = async (source: boolean, reset: boolean = false) => {
		await invoke('get_files', { source, reset }).then((res: any) => {
			if (source) {
				sendFiles = res.files as LocalFile[];
				sendCwd = res.pwd as string;
			} else {
				recvFiles = res.files as LocalFile[];
				recvCwd = res.pwd as string;
			}
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
		if (algo === null) {
			notify.warning('Algorithm is not selected', 3000);
			return;
		} else if (algoStr.startsWith('block:') && mode === null) {
			notify.warning('Block mode is not selected', 3000);
			return;
		}
		if (key === null) {
			notify.warning('Key is not selected', 3000);
			return;
		}

		invoke('send_file', {
			request: {
				algorithm: algoStr,
				mode: mode ? modeStr : undefined,
				key: key.key,
				iv: key.iv,
				hash_algo: hashStr,
				padding: mode ? 'pkcs7' : undefined
			},
			file: filename,
			ip: sendIp,
			port: sendPort
		})
			.then(() => {
				notify.success('File sent successfully.', 3000);
			})
			.catch((err: any) => {
				notify.error(err.message, 3000);
			});
	};

	const onKeyAction = async () => {
		if (!key) return;

		invoke('send_key', {
			key: {
				key: key.key,
				iv: key.iv
			},
			ip: sendIp,
			port: sendPort
		})
			.then(() => {
				notify.success('Key sent successfully.', 3000);
			})
			.catch((err: any) => {
				notify.error(err.message, 3000);
			});
	};

	const loadNetKeys = () => {
		invoke('get_network_keys').then((res: any) => {
			networkKeys.clear();
			Object.entries(res).forEach((entry) => {
				networkKeys.set(entry[0], entry[1] as number[]);
			});
		});
	};

	const listenFor = (files: boolean) => {
		const listening = files ? fileListening : keyListening;
		if (listening) return;

		const command = files ? 'start_file_listening' : 'start_key_listening';
		invoke(command, {
			port: files ? recvPort : recvKeyPort
		})
			.then(() => {
				if (files) fileListening = true;
				else keyListening = true;
			})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const stopListening = (file: boolean) => {
		const listening = file ? fileListening : keyListening;
		if (!listening) return;

		invoke(file ? 'stop_file_listening' : 'stop_key_listening')
			.then(() => {
				if (file) fileListening = false;
				else keyListening = false;
			})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	const approveDenyFile = (sockAddr: string, accept: boolean) => {
		invoke(accept ? 'approve_incoming' : 'deny_incoming', { addr: sockAddr })
			.then(() => {})
			.catch((err) => {
				notify.error(err, 3000);
			});
	};

	onMount(() => {
		loadFiles(true, true);
		loadFiles(false, true);

		const unlisteners: Array<() => void> = [];

		const setupListeners = async () => {
			unlisteners.push(
				await listen<ProgressFile>('network:send:start', (event) => {
					if (!sendingFiles.has(event.payload.filename)) {
						sendingFiles.set(event.payload.filename, {
							...event.payload,
							size: event.payload.total
						});
					}
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('network:send:done', (event) => {
					if (sendingFiles.has(event.payload.filename)) {
						sendingFiles.delete(event.payload.filename);
					}
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('network:send:progress', (event) => {
					if (sendingFiles.has(event.payload.filename)) {
						sendingFiles.set(event.payload.filename, {
							...event.payload,
							size: event.payload.total
						});
					}
				})
			);

			unlisteners.push(
				await listen<CryptoError>('network:send:error', (event) => {
					if (sendingFiles.has(event.payload.filename)) {
						sendingFiles.delete(event.payload.filename);
					}
					notify.error(event.payload.err, 3000);
				})
			);

			unlisteners.push(
				await listen('network:key:saved', (event) => {
					loadNetKeys();
					keyListening = false;
					notify.info(`Received key from ${event.payload}`, 3000);
				})
			);

			unlisteners.push(
				await listen('network:error', (event) => {
					notify.error(event.payload as any, 3000);
				})
			);

			unlisteners.push(
				await listen<CryptoError>('network:recv:error', (event) => {
					recvQueue = recvQueue.filter(
						(pf) => pf.sockAddr !== event.payload.filename
					);
					if (receivingFiles.has(event.payload.filename)) {
						receivingFiles.delete(event.payload.filename);
					}
					notify.error(event.payload.err, 3000);
				})
			);

			unlisteners.push(
				await listen<PendingFile>('network:recv:pending', (event) => {
					recvQueue.push(event.payload);
				})
			);

			unlisteners.push(
				await listen<string>('network:recv:denied', (event) => {
					recvQueue = recvQueue.filter((pf) => pf.sockAddr !== event.payload);
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('network:recv:start', (event) => {
					recvQueue = recvQueue.filter(
						(pf) => pf.filename !== event.payload.filename
					);
					if (!receivingFiles.has(event.payload.filename)) {
						receivingFiles.set(event.payload.filename, {
							...event.payload,
							size: event.payload.total
						});
					}
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('network:recv:done', (event) => {
					if (receivingFiles.has(event.payload.filename)) {
						receivingFiles.delete(event.payload.filename);
					}
					loadFiles(false);
				})
			);

			unlisteners.push(
				await listen<ProgressFile>('network:recv:progress', (event) => {
					if (receivingFiles.has(event.payload.filename)) {
						receivingFiles.set(event.payload.filename, {
							...event.payload,
							size: event.payload.total
						});
					}
				})
			);
		};

		setupListeners();

		return () => {
			unlisteners.forEach((fn) => fn());
		};
	});
</script>

<div class="flex h-full min-h-0 flex-1">
	<div class="flex h-full min-h-0 flex-1 flex-col gap-2 overflow-hidden p-4">
		<div class="flex w-full flex-wrap items-center px-5 py-1">
			<p class="mr-3">Choose algorithm:</p>
			<Select.Root
				type="single"
				bind:value={algoStr}
				onValueChange={onAlgoSelect}
			>
				<Select.Trigger
					class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40"
				>
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
				>
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
			{/if}
		</div>
		<div class="flex w-full flex-wrap items-center px-2 py-1">
			<p class="mx-3">Choose hash:</p>
			<Select.Root type="single" bind:value={hashStr}>
				<Select.Trigger
					class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40"
				>
					{triggerContentHash}
				</Select.Trigger>
				<Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
					<Select.Group>
						<Select.Item
							value=""
							label="None"
							disabled={hashStr === ''}
							class="hover:text-fg-0 hover:bg-bg-3/50"
						>
							None
						</Select.Item>
						{#each hashModes as hashMode}
							<Select.Item
								value={hashMode.value}
								label={hashMode.label}
								disabled={hashStr === hashMode.value}
								class="hover:text-fg-0 hover:bg-bg-3/50"
							>
								{hashMode.label}
							</Select.Item>
						{/each}
					</Select.Group>
				</Select.Content>
			</Select.Root>
		</div>
		<div class="flex w-full flex-wrap items-center gap-3 px-5 py-1">
			<p>IP Address:</p>
			<input
				type="text"
				class="border-bg-5 data-[placeholder]:text-fg-3 w-60 rounded-md border px-3 py-1 outline-none"
				placeholder="127.0.0.1"
				bind:value={sendIp}
			/>
			<p>Port:</p>
			<input
				type="number"
				class="border-bg-5 data-[placeholder]:text-fg-3 w-40
            	[appearance:textfield] rounded-md border px-3 py-1 outline-none [&::-webkit-inner-spin-button]:appearance-none 
				[&::-webkit-outer-spin-button]:appearance-none"
				placeholder="443"
				bind:value={sendPort}
			/>
		</div>
		<div class="flex w-full flex-wrap items-center px-5 py-1">
			<p class="">Key:</p>
			<p class="mx-2 {key !== null ? 'text-primary font-black' : 'text-fg-4'}">
				{key?.label ?? 'No key selected'}
			</p>
			<KeyDialog
				{algo}
				{mode}
				bind:outputKey={key}
				{onKeySet}
				operation={'enc'}
			/>
			<button
				disabled={key === null ||
					sendIp.length === 0 ||
					Number.isNaN(parseInt(sendPort))}
				onclick={onKeyAction}
				class="bg-primary hover:bg-primary/60 disabled:bg-bg-5 disabled:text-bg-1 dark:disabled:text-fg-3
                text-bg-0 dark:text-fg-0 ml-3 cursor-pointer rounded-sm px-4 py-3 transition-colors duration-300"
				>Send Key</button
			>
			<button
				disabled={key === null ||
					selectedFile === null ||
					sendingFiles.size !== 0 ||
					sendIp.length === 0 ||
					Number.isNaN(parseInt(sendPort))}
				onclick={() => selectedFile && onFileAction(selectedFile?.filename)}
				class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 disabled:bg-bg-5 disabled:text-bg-1
				dark:disabled:text-fg-3 ml-auto flex cursor-pointer gap-2 rounded-sm px-7 py-2 transition-colors duration-300"
				><HardDriveUploadIcon /> Send File</button
			>
		</div>
		<div class="min-h-0 flex-1">
			<FileExplorer
				class="h-full w-full"
				bind:pwd={sendCwd}
				files={sendFiles}
				label={`Send Directory`}
				bind:locked={sendLocked}
				bind:selectedFile
				lockIcon={LockIcon}
				unlockIcon={LockOpenIcon}
				onGoBack={async () => await goDirBack(true)}
				onChangeDir={async (dir) => await changeDir(dir, true)}
				{onFileAction}
				onStopProcessingFile={() => {}}
				onSetAbsolutePath={async (newDir) =>
					await setAbsolutePath(newDir, true)}
				onLockChange={async () => true}
				onRefresh={() => loadFiles(true)}
				constFilter={undefined}
				processingFiles={sendingFilesArray}
				pendingFiles={[]}
				onAcceptRejectFile={() => {}}
			/>
		</div>
	</div>
	<div class="flex h-full min-h-0 flex-1 flex-col overflow-hidden p-4">
		<div class="flex w-full flex-wrap items-center gap-3 px-5 py-1">
			<p>File Listen Port:</p>
			<input
				type="number"
				class="border-bg-5 data-[placeholder]:text-fg-3 disabled:text-fg-2 w-40 [appearance:textfield] 
				rounded-md border px-3 py-1 outline-none [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
				disabled={fileListening}
				placeholder="443"
				bind:value={recvPort}
			/>
			<p>Key Listen Port:</p>
			<input
				type="number"
				class="border-bg-5 data-[placeholder]:text-fg-3 disabled:text-fg-2 w-40 [appearance:textfield] 
				rounded-md border px-3 py-1 outline-none [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none"
				disabled={keyListening}
				placeholder="443"
				bind:value={recvKeyPort}
			/>
		</div>
		<div class="grid w-full grid-cols-3">
			<div class="grid grid-rows-2">
				<div class="flex flex-col items-center justify-center gap-3">
					{#if fileListening}
						<p>Listening for Files on :{recvPort}...</p>
						<button
							class="bg-error hover:bg-error/70 text-bg-0 dark:text-fg-0 cursor-pointer rounded-md px-3 py-2 transition-colors duration-200"
							onclick={() => stopListening(true)}>Stop Listening</button
						>
					{:else}
						<p>Listen for Files</p>
						<button
							disabled={Number.isNaN(parseInt(recvPort)) || !recvLocked}
							class="bg-primary hover:bg-primary/70 disabled:bg-bg-5 text-bg-0 dark:text-fg-0 disabled:dark:text-fg-3
                            disabled:text-bg-3 cursor-pointer rounded-md px-3 py-2 transition-colors duration-200"
							onclick={() => listenFor(true)}>Listen for Files</button
						>
					{/if}
				</div>
				<div class="flex flex-col items-center justify-center gap-3">
					{#if keyListening}
						<p>Listening for Key on :{recvKeyPort}...</p>
						<button
							class="bg-error hover:bg-error/70 text-bg-0 dark:text-fg-0 cursor-pointer rounded-md px-3 py-2 transition-colors duration-200"
							onclick={() => stopListening(false)}>Stop Listening</button
						>
					{:else}
						<p>Listen for Key</p>
						<button
							disabled={Number.isNaN(parseInt(recvKeyPort))}
							class="bg-primary hover:bg-primary/70 disabled:bg-bg-5 text-bg-0 dark:text-fg-0 disabled:dark:text-fg-3
                            disabled:text-bg-3 cursor-pointer rounded-md px-3 py-2 transition-colors duration-200"
							onclick={() => listenFor(false)}>Listen for Key</button
						>
					{/if}
				</div>
			</div>
			<div class="col-span-2 flex h-60 flex-col p-4">
				<div class="bg-bg-1 text-fg-2 flex px-2 py-1">
					<p class="flex-1">IP Address</p>
					<p class="flex-1">Key Size</p>
				</div>
				<div class="h-full w-full overflow-auto">
					{#each networkKeys.entries() as netKey}
						<div class="hover:bg-bg-1 odd:bg-bg-1/50 relative flex px-3 py-1">
							<p class="flex-1">{netKey[0]}</p>
							<p class="flex-1">
								{netKey[1][0]}{netKey[1][1] > 0 ? `+ ${netKey[1][1]}` : ''}B
							</p>
							<XCircleIcon class="text-error absolute right-4 cursor-pointer" />
						</div>
					{/each}
				</div>
			</div>
		</div>
		<div class="min-h-0 flex-1">
			<FileExplorer
				class="h-full w-full"
				pwd={recvCwd}
				files={recvFiles}
				label={`Receive Directory`}
				bind:locked={recvLocked}
				selectedFile={null}
				lockIcon={LockIcon}
				unlockIcon={LockOpenIcon}
				onGoBack={async () => await goDirBack(false)}
				onChangeDir={async (dir) => await changeDir(dir, false)}
				{onFileAction}
				onStopProcessingFile={() => {}}
				onSetAbsolutePath={async (newDir) =>
					await setAbsolutePath(newDir, false)}
				onLockChange={async () => true}
				onRefresh={() => loadFiles(false)}
				constFilter={undefined}
				processingFiles={receivingFilesArray}
				pendingFiles={recvQueue}
				onAcceptRejectFile={approveDenyFile}
			/>
		</div>
	</div>
</div>
