<script lang="ts">
	import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
    import { HardDriveUploadIcon, LockIcon, LockOpenIcon } from "@lucide/svelte";
    import * as Select from "$lib/components/ui/select/index";
	import { blockCiphers, blockModes, streamCiphers, type Key } from "$lib/types/crypto";
	import KeyDialog from "$lib/components/ui/key-dialog/KeyDialog.svelte";
	import type { LocalFile, ProgressFile } from "$lib/components/ui/file-explorer/utils";
	import { SvelteMap } from "svelte/reactivity";
	import { invoke } from "@tauri-apps/api/core";
	import { onMount } from "svelte";
	import { listen } from "@tauri-apps/api/event";

    let sendFiles: LocalFile[] = $state([]);
    let sendCwd: string = $state('');

    let recvFiles: LocalFile[] = $state([]);
    let recvCwd: string = $state('');

    let sendingFiles = $state<SvelteMap<string, ProgressFile>>(new SvelteMap());

    let algoStr = $state('');
    let modeStr = $state('');

    let sendIp = $state('');
    let sendPort = $state('');

    let recvIp = $state('');
    let recvPort = $state('');

    let key = $state<Key|null>(null);
    let cachedKeys = $state<{ [algoMode: string] : Key}>({});

    let recvLocked = $state(false);
    let sendLocked = $state(false);

    let algo = $derived.by(() => {
        return [...blockCiphers, ...streamCiphers].find(v => v.value === algoStr) ?? null
    });
    let mode = $derived.by(() => {
        return blockModes.find(v => v.value === modeStr) ?? null
    });

    let sendingFilesArray = $derived.by(() => {
        return Array.from(sendingFiles.values());
    });

    const triggerContent = $derived(
        streamCiphers.find(c => c.value === algoStr)?.label ??
        blockCiphers.find(c => c.value === algoStr)?.label ?? "Select Cipher"
    );

    const triggerContentMode = $derived(
        blockModes.find(m => m.value === modeStr)?.label ?? "Select Mode"
    );

    const onKeySet = (newKey: Key) => {
        cachedKeys[algoStr + ":" + modeStr] = newKey;
    };

    const onAlgoSelect = (newAlgo: string) => {
        if (cachedKeys[newAlgo + ":" + modeStr])
            key = cachedKeys[newAlgo + ":" + modeStr];
        else
            key = null;
    };

    const onModeSelect = (newMode: string) => {
        if (cachedKeys[algoStr + ":" + newMode])
            key = cachedKeys[algoStr + ":" + newMode];
        else
            key = null;
    };

    const loadFiles = async (source: boolean) => {
        await invoke('get_files', { source }).then((res: any) => {
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
            const res : boolean = await invoke('change_dir', { newDir, source });
            if (res) await loadFiles(source);

            return res;
        } catch(err) {
            return false;
        }
    };

    const goDirBack = async (source: boolean) => {
        try {
            const res : boolean = await invoke('go_dir_back', { source });
            if (res) await loadFiles(source);

            return res;
        } catch(err) {
            return false;
        }
    };

    const setAbsolutePath = async (newDir: string, source: boolean) => {
        try {
            const res : boolean = await invoke('set_current_dir', { newDir, source });
            if (res) await loadFiles(source);

            return res;
        } catch(err) {
            return false;
        }
    };

    const onFileAction = async (filename: string) => {
        if (!key) return;
        
        invoke("send_file", {
            request: {
                algorithm: algoStr,
                mode: mode ? modeStr : undefined,
                key: key.key,
                iv: key.iv
            },
            file: filename,
            ip: sendIp,
            port: sendPort
        })
        .then(() => {

        })
        .catch(err => {
            console.error(err);
        });
    };

    const onKeyAction = async () => {
        if (!key) return;

        invoke("send_key", {
            key: {
                key: key.key,
                iv: key.iv
            },
            ip: sendIp,
            port: sendPort
        })
        .catch(err => {
            console.error(err)
        });
    };

    onMount(() => {
        loadFiles(true);
        loadFiles(false);

        const unlisteners: Array<() => void> = [];

        const setupListeners = async () => {
            unlisteners.push(await listen<ProgressFile>("network:send:start", (event) => {
                if (!sendingFiles.has(event.payload.filename)) {
                    sendingFiles.set(event.payload.filename, {
                        ...event.payload,
                        size: event.payload.total
                    });
                }
            }));

            unlisteners.push(await listen<ProgressFile>("network:send:done", (event) => {
                if (sendingFiles.has(event.payload.filename)) {
                    sendingFiles.delete(event.payload.filename);
                }
            }));

            unlisteners.push(await listen<ProgressFile>("network:send:progress", (event) => {
                if (sendingFiles.has(event.payload.filename)) {
                    sendingFiles.set(event.payload.filename, {
                        ...event.payload,
                        size: event.payload.total
                    });
                }
            }));

            unlisteners.push(await listen<ProgressFile>("network:error", (event) => {
                console.error(event.payload);
            }));
        };

        setupListeners();

        return () => {
            unlisteners.forEach(fn => fn());
        };
    });
</script>

<div class="flex flex-1 min-h-0 h-full">
    <div class="flex-1 p-4 overflow-hidden flex flex-col gap-2 h-full min-h-0">
        <div class="w-full flex flex-wrap items-center px-5 py-1">
            <p class="mr-3">Choose algorithm:</p>
            <Select.Root type="single" bind:value={algoStr} onValueChange={onAlgoSelect}>
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
            {#if algoStr.startsWith("block:")}
                <p class="mx-3">Choose mode:</p>
                <Select.Root type="single" bind:value={modeStr} onValueChange={onModeSelect}>
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
            {/if}
        </div>
        <div class="w-full flex flex-wrap items-center px-5 py-1 gap-3">
            <p>IP Address:</p>
            <input type="text" class="outline-none border border-bg-5 data-[placeholder]:text-fg-3 w-60 py-1 px-3 rounded-md" placeholder="127.0.0.1" bind:value={sendIp} />
            <p>Port:</p>
            <input type="number" class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
            outline-none border border-bg-5 data-[placeholder]:text-fg-3 w-40 py-1 px-3 rounded-md" placeholder="443" bind:value={sendPort} />
            <button>
                <LockIcon class="bg-bg-3 hover:bg-bg-4 rounded-md p-2 size-10 mr-5 cursor-pointer text-fg-2" />
            </button>
        </div>
        <div class="w-full flex flex-wrap items-center px-5 py-1">
            <p class="">Key:</p>
            <p class="mx-2 {key !== null ? "text-primary font-black" : "text-fg-4"}">{key?.label ?? "No key selected"}</p>
            <KeyDialog algo={algo} mode={mode} bind:outputKey={key} onKeySet={onKeySet} operation={'enc'} />
            <button
                onclick={onKeyAction}
                class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 px-4 py-2 rounded-sm ml-3 cursor-pointer transition-colors duration-300"
            >Send Key</button>
            <button
                class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 px-7 py-2 
                flex gap-2 rounded-sm ml-auto cursor-pointer transition-colors duration-300"
            ><HardDriveUploadIcon /> Send File</button>
        </div>
        <div class="flex-1 min-h-0">
            <FileExplorer 
                class="h-full w-full" 
                bind:pwd={sendCwd}
                files={sendFiles}
                label={`Send Directory`} 
                bind:locked={sendLocked}
                lockIcon={LockIcon}
                unlockIcon={LockOpenIcon}
                onGoBack={async () => await goDirBack(true)}
                onChangeDir={async dir => await changeDir(dir, true)}
                onFileAction={onFileAction}
                onStopProcessingFile={() => {}}
                onSetAbsolutePath={async newDir => await setAbsolutePath(newDir, true)}
                onLockChange={async () => true}
                onRefresh={() => loadFiles(true)}
                constFilter={undefined}
                processingFiles={sendingFilesArray}
            />
        </div>
    </div>
    <div class="flex-1 p-4 h-full min-h-0 overflow-hidden flex flex-col">
        <div class="w-full flex flex-wrap items-center px-5 py-1 gap-3">
            <p>IP Address:</p>
            <input type="text" class="outline-none border border-bg-5 data-[placeholder]:text-fg-3 w-60 py-1 px-3 rounded-md" placeholder="127.0.0.1" bind:value={recvIp} />
            <p>Port:</p>
            <input type="number" class="[appearance:textfield] [&::-webkit-outer-spin-button]:appearance-none [&::-webkit-inner-spin-button]:appearance-none
            outline-none border border-bg-5 data-[placeholder]:text-fg-3 w-40 py-1 px-3 rounded-md" placeholder="443" bind:value={recvPort} />
            <button>
                <LockIcon class="bg-bg-3 hover:bg-bg-4 rounded-md p-2 size-10 mr-5 cursor-pointer text-fg-2" />
            </button>
        </div>
        <div class="w-full grid grid-cols-3">
            <div>RED</div>
            <div class="col-span-2">
                BLUE
            </div>
        </div>
        <div class="flex-1 min-h-0">
            <FileExplorer 
                class="h-full w-full"
                pwd={recvCwd}
                files={recvFiles} 
                label={`Receive Directory`} 
                bind:locked={recvLocked}
                lockIcon={LockIcon}
                unlockIcon={LockOpenIcon}
                onGoBack={async () => await goDirBack(false)}
                onChangeDir={async dir => await changeDir(dir, false)}
                onFileAction={onFileAction}
                onStopProcessingFile={() => {}}
                onSetAbsolutePath={async newDir => await setAbsolutePath(newDir, false)}
                onLockChange={async () => true}
                onRefresh={() => loadFiles(false)}
                constFilter={undefined}
                processingFiles={[]}
            />
        </div>
    </div>
</div>