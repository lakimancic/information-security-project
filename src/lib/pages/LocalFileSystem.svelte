<script lang="ts">
	import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";
	import KeyDialog from "$lib/components/ui/key-dialog/KeyDialog.svelte";
	import * as Select from "$lib/components/ui/select/index";
	import { type CipherTag, type Key } from "$lib/types/crypto";
	import { EyeOffIcon, KeySquareIcon, LockIcon, LockOpenIcon, ScanEyeIcon, XIcon } from "@lucide/svelte";
    import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";

    let sourceFiles: LocalFile[] = $state([]);
    let sourceCwd: string = $state('');

    let destFiles: LocalFile[] = $state([]);
    let destCwd: string = $state('');

    let algoStr = $state('');
    let modeStr = $state('');

    let operation = $state<'dec'|'enc'>('enc');
    let key = $state<Key|null>(null);
    let cachedKeys = $state<{ [algoMode: string] : Key}>({});

    let destLocked = $state(false);
    let sourceWatch = $state(false);

    const streamCiphers = [
        { value: "stream:a5/1", label: "A5/1" }
    ];

    const blockCiphers : CipherTag[] = [
        { value: "block:xtea", label: "XTEA" },
        { value: "block:aes256", label: "AES-256" }
    ];

    const blockModes : CipherTag[] = [
        { value: "mode:ofb", label: "OFB" }
    ];

    let algo = $derived.by(() => {
        return [...blockCiphers, ...streamCiphers].find(v => v.value === algoStr) ?? null
    });
    let mode = $derived.by(() => {
        return blockModes.find(v => v.value === modeStr) ?? null
    });

    const triggerContent = $derived(
        streamCiphers.find(c => c.value === algoStr)?.label ??
        blockCiphers.find(c => c.value === algoStr)?.label ?? "Select Cipher"
    );

    const triggerContentMode = $derived(
        blockModes.find(m => m.value === modeStr)?.label ?? "Select Mode"
    );

    const loadFiles = async (source: boolean) => {
        await invoke('get_files', { source }).then((res: any) => {
            if (source) {
                sourceFiles = res.files as LocalFile[];
                sourceCwd = res.pwd as string;
            } else {
                destFiles = res.files as LocalFile[];
                destCwd = res.pwd as string;
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
        if (cachedKeys[algo + ":" + newMode])
            key = cachedKeys[algo + ":" + newMode];
        else
            key = null;
    };

    onMount(() => {
        loadFiles(true);
        loadFiles(false);
    });
</script>

<div class="flex flex-wrap items-center px-5 py-1">
    <p class="mr-3">Choose algorithm:</p>
    <Select.Root type="single" bind:value={algoStr} onValueChange={onAlgoSelect} disabled={operation === 'dec'}>
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
        <Select.Root type="single" bind:value={modeStr} onValueChange={onModeSelect} disabled={operation === 'dec'}>
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
    <p class="ml-auto">Key:</p>
    <p class="mx-2 {key !== null ? "text-primary font-black" : "text-fg-4"}">{key?.label ?? "No key selected"}</p>
    <KeyDialog algo={algo} mode={mode} bind:outputKey={key} onKeySet={onKeySet} operation={operation} />
    <p class="mr-2">Operation:</p>
    <div class="flex border border-bg-4 p-1 rounded-sm gap-2">
        <button 
            class="border-2 {operation === 'dec' ? 'border-bg-4' : 'border-primary'} p-2 rounded-sm cursor-pointer"
            onclick={() => operation = 'enc'}
        >Encryption</button>
        <button 
            class="border-2 {operation === 'enc' ? 'border-bg-4' : 'border-primary'} p-2 rounded-sm cursor-pointer"
            onclick={() => operation = 'dec'}
        >Decryption</button>
    </div>
    <button
        class="bg-primary hover:bg-primary/60 text-bg-0 dark:text-fg-0 px-4 py-3 rounded-sm ml-3 cursor-pointer transition-colors duration-300"
    >{operation === 'enc' ? 'Encrypt' : 'Decrypt'}</button>
</div>
<div class="flex flex-1 min-h-0">
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer 
            class="" 
            bind:pwd={sourceCwd}
            files={sourceFiles} 
            label="Source Directory" 
            bind:locked={sourceWatch}
            lockIcon={ScanEyeIcon}
            unlockIcon={EyeOffIcon}
            onGoBack={async () => await goDirBack(true) }
            onChangeDir={async dir => await changeDir(dir, true)}
            onFileAction={() => {}}
            onStopProcessingFile={() => {}}
            onSetAbsolutePath={async newDir => await setAbsolutePath(newDir, true)}
            onLockChange={() => true}
            onRefresh={() => loadFiles(true)}
            constFilter={operation === 'dec' ? /^.*\.enc$/ : undefined}
            processingFiles={[]}
        />
    </div>
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer 
            class="" 
            pwd={destCwd} 
            files={destFiles} 
            label={`Destination Directory${destLocked ? " (Locked)" : ""}`} 
            bind:locked={destLocked}
            lockIcon={LockIcon}
            unlockIcon={LockOpenIcon}
            onGoBack={() => goDirBack(false) }
            onChangeDir={dir => changeDir(dir, false)}
            onFileAction={() => {}}
            onStopProcessingFile={() => {}}
            onSetAbsolutePath={async newDir => await setAbsolutePath(newDir, false)}
            onLockChange={() => true}
            onRefresh={() => loadFiles(false)}
            constFilter={operation === 'enc' ? /^.*\.enc$/ : undefined}
            processingFiles={[
                { filename: "test.enc", done: 10, total: 40, size: 500 }
            ]}
        />
    </div>
</div>