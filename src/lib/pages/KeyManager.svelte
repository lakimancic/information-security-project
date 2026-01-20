<script lang="ts">
    import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";
	import { notify } from "$lib/components/ui/notifications/store";
    import * as Select from "$lib/components/ui/select/index";
	import { blockCiphers, blockModes, streamCiphers, type ShortKey } from "$lib/types/crypto";
	import { sizesToAlgorithm } from "$lib/utils/crypto";
	import { FolderOpenIcon, KeyIcon, LockIcon, LockOpenIcon, SaveAllIcon, Trash2Icon } from "@lucide/svelte";
    import { invoke } from '@tauri-apps/api/core';
	import { Dialog, Separator } from "bits-ui";
    import { onMount } from "svelte";

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

    let selectedFile : LocalFile|null = $state(null);

    const triggerContent = $derived(
        streamCiphers.find(c => c.value === algoStr)?.label ??
        blockCiphers.find(c => c.value === algoStr)?.label ?? "Select Cipher"
    );

    const triggerContentMode = $derived(
        blockModes.find(m => m.value === modeStr)?.label ?? "Select Mode"
    );

    const loadFiles = async (reset: boolean = false) => {
        await invoke('get_files', { source: true, reset }).then((res: any) => {
            files = res.files as LocalFile[];
            pwd = res.pwd as string;
        });
    };

    const listKeys = async () => {
        await invoke<ShortKey[]>('list_keys').then(res => {
            keys = res;
        });
    };

    const handleGenerateKey = async () => {
        if (algoStr === '') {
            errorMsg = 'Algorithm is not selected.';
            return;
        }

        if (algoStr.startsWith("block:") && modeStr === '') {
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
            errorMsg = 'Passwords doesn\'t match';
            return;
        }

        invoke("generate_new_key", { algorithm: algoStr, mode: modeStr, name: genName, password: genPass })
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
        .catch(err => {
            notify.error(err, 3000);
        });
    };

    const removeKey = async (name: string) => {
        await invoke('remove_key', { name })
        .then(() => {
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
        .catch(err => {
            notify.error(err, 3000);
        })
    };

    const changeDir = async (newDir: string) => {
        try {
            const res : boolean = await invoke('change_dir', { newDir, source: true });
            if (res) await loadFiles(true);

            return res;
        } catch(err) {
            return false;
        }
    };

    const goDirBack = async () => {
        try {
            const res : boolean = await invoke('go_dir_back', { source: true });
            if (res) await loadFiles(true);

            return res;
        } catch(err) {
            return false;
        }
    };

    const setAbsolutePath = async (newDir: string) => {
        try {
            const res : boolean = await invoke('set_current_dir', { newDir, source: true });
            if (res) await loadFiles(true);

            return res;
        } catch(err) {
            return false;
        }
    };

    onMount(() => {
        loadFiles(true);
        listKeys();
    });
</script>

<div class="min-h-0 grid grid-cols-2 h-full">
    <div class="p-4 min-h-0 overflow-hidden flex flex-col">
        <FileExplorer 
            class="h-full" 
            bind:pwd={pwd}
            files={files} 
            label="Look for Keys in directory" 
            locked={filesLocked}
            bind:selectedFile={selectedFile}
            lockIcon={LockIcon}
            unlockIcon={LockOpenIcon}
            onGoBack={async () => await goDirBack() }
            onChangeDir={async dir => await changeDir(dir)}
            onFileAction={() => {}}
            onStopProcessingFile={() => {}}
            onSetAbsolutePath={async newDir => await setAbsolutePath(newDir)}
            onLockChange={async () => true}
            onRefresh={() => loadFiles(true)}
            constFilter={/^.*\.keys$/}
            processingFiles={[]}
            pendingFiles={[]}
            onAcceptRejectFile={() => {}}
        />
    </div>
    <div class="p-4 flex flex-col gap-2 min-h-0">
        <div class="flex justify-around gap-5 py-5">
            <button class="flex px-4 py-3 gap-3 text-xl items-center bg-primary hover:bg-primary/60 cursor-pointer rounded-md
                transition-all duration-200 text-bg-0 dark:text-fg-0 disabled:bg-bg-5 disabled:text-bg-1 dark:disabled:text-fg-3"
                disabled={selectedFile === null}
                onclick={() => loadKeys()}
            >
                Load Keys <FolderOpenIcon />
            </button>
            <Dialog.Root bind:open={dialogOpen}>
                <Dialog.Trigger class="flex px-4 py-3 gap-3 text-xl items-center bg-primary hover:bg-primary/60 cursor-pointer rounded-md
                    transition-all duration-200 text-bg-0 dark:text-fg-0">
                    Generate New <KeyIcon />
                </Dialog.Trigger>
                <Dialog.Portal>
                    <Dialog.Overlay
                        class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-bg-0/80"
                    />
                    <Dialog.Content
                        class="bg-bg-1 text-fg-0 border border-bg-4 rounded-md data-[state=open]:animate-in data-[state=closed]:animate-out 
                        data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 
                        outline-hidden fixed left-[50%] top-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] p-5 sm:max-w-[490px] md:w-full"
                    >
                        <Dialog.Title
                            class="flex w-full items-center justify-center text-xl font-semibold tracking-tight"
                        >
                            Generate New Key
                        </Dialog.Title>
                        <Separator.Root class="bg-bg-4 -mx-5 mb-6 mt-5 block h-px" />
                        <Dialog.Description class="text-fg-3 text-sm mt-2 mb-6">
                            Generate new key for specific algorithms and block modes. All keys are generated by secure pseudo random generator.
                        </Dialog.Description>
                        <div class="flex items-center gap-4 py-2">
                            <p>Choose algorithm:</p>
                            <Select.Root type="single" bind:value={algoStr}>
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
                        </div>
                        {#if algoStr.startsWith("block:")}
                        <div class="flex items-center gap-4 py-2">
                            <p>Choose block mode:</p>
                            <Select.Root type="single" bind:value={modeStr}>
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
                        </div>
                        {/if}
                        <div class="flex items-center gap-4 py-2">
                            <p>Enter new key name:</p>
                            <input 
                                type="text" 
                                name="key_name" 
                                id="keyName"
                                placeholder="key_name"
                                class="outline-none border border-bg-5 min-w-40 py-1.5 px-3 rounded-md placeholder:text-fg-0/40"
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
                                class="outline-none border border-bg-5 min-w-40 py-1.5 px-3 rounded-md placeholder:text-fg-0/40"
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
                                class="outline-none border border-bg-5 min-w-40 py-1.5 px-3 rounded-md placeholder:text-fg-0/40"
                                bind:value={genPassRep}
                            />
                        </div>
                        <p class="text-center text-error">{errorMsg}</p>
                        <div class="flex w-full justify-end mt-5">
                        <button
                            class="bg-primary px-4 py-3 font-semibold rounded-md cursor-pointer hover:bg-primary/70 transition-colors duration-300"
                            onclick={handleGenerateKey}
                        >Generate Key</button>
                    </div>
                    </Dialog.Content>
                </Dialog.Portal>
            </Dialog.Root>
            <button class="flex px-4 py-3 gap-3 text-xl items-center bg-primary hover:bg-primary/60 cursor-pointer rounded-md
                transition-all duration-200 text-bg-0 dark:text-fg-0 disabled:bg-bg-5 disabled:text-bg-1 dark:disabled:text-fg-3"
                disabled={saveFile.length === 0}
                onclick={saveKeys}
            >
                Save Keys <SaveAllIcon />
            </button>
        </div>
        <div class="flex py-3 items-center">
            <p class="mr-5">Save Keys file name: </p>
            <input type="text" class="outline-none border border-bg-5 data-[placeholder]:text-fg-3 w-60 py-1 px-3 rounded-md" 
                placeholder="data" bind:value={saveFile} />
            <p>.keys</p>
        </div>
        <div class="m-7 border flex-1 border-fg-5/50 font-semibold min-h-0 relative flex flex-col">
            <div class="grid grid-cols-[3fr_2fr_4fr] sticky top-0 w-full bg-bg-0 z-10">
                {#each ["Key name", "Key size", "Possible Algorithms"] as colName, colIndex}
                    <p class="px-3 py-2 border-b {colIndex < 2 && 'border-r'} border-fg-5/50 bg-fg-3/10">
                        {colName}
                    </p>
                {/each}
            </div>
            <div class="grid grid-cols-[3fr_2fr_4fr] auto-rows-max min-h-0 overflow-auto items-start content-start relative z-8">
                {#each keys as key}
                    <p class="px-3 py-2 border-b border-r border-fg-5/50 truncate">{key.name}</p>
                    <p class="px-3 py-2 border-b border-r border-fg-5/50 truncate">{key.keySize}{key.ivSize > 0 ? `+${key.ivSize}` : ''} B</p>
                    <p class="px-3 py-2 text-primary border-b border-fg-5/50 relative flex items-center truncate">
                        {sizesToAlgorithm(key.keySize, key.ivSize).join(", ") ?? '...'}
                        <button 
                            class="absolute right-5 cursor-pointer text-error hover:text-error/80"
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