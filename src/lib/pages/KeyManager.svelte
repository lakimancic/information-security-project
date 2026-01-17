<script lang="ts">
    import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";
	import type { ShortKey } from "$lib/types/crypto";
	import { sizesToAlgorithm } from "$lib/utils/crypto";
	import { FolderOpenIcon, KeyIcon, LockIcon, LockOpenIcon, SaveAllIcon, Trash2Icon } from "@lucide/svelte";
    import { invoke } from '@tauri-apps/api/core';
    import { onMount } from "svelte";

    let files: LocalFile[] = $state([]);
    let pwd: string = $state('');
    let filesLocked = $state(false);

    let saveFile = $state('');

    let keys: ShortKey[] = $state([]);

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

    const loadKeys = async (filename: string) => {
        await invoke('load_keys', { filename })
        .then(() => {
            listKeys();
        })
        .catch(err => {
            console.error(err);
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

        })
        .catch(err => {
            console.error(err);
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
                transition-all duration-200 text-bg-0 dark:text-fg-0"
            >
                Load Keys <FolderOpenIcon />
            </button>
            <button class="flex px-4 py-3 gap-3 text-xl items-center bg-primary hover:bg-primary/60 cursor-pointer rounded-md
            transition-all duration-200 text-bg-0 dark:text-fg-0">
                Generate New <KeyIcon />
            </button>
            <button class="flex px-4 py-3 gap-3 text-xl items-center bg-primary hover:bg-primary/60 cursor-pointer rounded-md
                transition-all duration-200 text-bg-0 dark:text-fg-0"
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