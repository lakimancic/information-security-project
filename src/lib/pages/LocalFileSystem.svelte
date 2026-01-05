<script lang="ts">
	import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";
	import * as Select from "$lib/components/ui/select/index";
	import { EyeOffIcon, LockIcon, LockOpenIcon, ScanEyeIcon } from "@lucide/svelte";
    import { invoke } from '@tauri-apps/api/core';
	import { onMount } from "svelte";

    let sourceFiles: LocalFile[] = $state([]);
    let sourceCwd: string = $state('');

    let destFiles: LocalFile[] = $state([]);
    let destCwd: string = $state('');

    let algo = $state('');
    let mode = $state('');
    let operation = $state<'dec'|'enc'>('enc');
    let destLocked = $state(false);
    let sourceWatch = $state(false);

    const streamCiphers = [
        { value: "stream:a5/1", label: "A5/1" }
    ];

    const blockCiphers = [
        { value: "block:xtea", label: "XTEA" }
    ];

    const blockModes = [
        { value: "mode:ofb", label: "OFB" }
    ];

    const triggerContent = $derived(
        streamCiphers.find(c => c.value === algo)?.label ??
        blockCiphers.find(c => c.value === algo)?.label ?? "Select Cipher"
    );

    const triggerContentMode = $derived(
        blockModes.find(m => m.value === mode)?.label ?? "Select Mode"
    );

    const loadFiles = (source: boolean) => {
        invoke('get_files', { source }).then((res: any) => {
            if (source) {
                sourceFiles = res.files as LocalFile[];
                sourceCwd = res.pwd as string;
            } else {
                destFiles = res.files as LocalFile[];
                destCwd = res.pwd as string;
            }
        });
    };

    const changeDir = (newDir: string, source: boolean) => {
        invoke('change_dir', { newDir, source }).then(() => {
            loadFiles(source);
        });
    };

    const goDirBack = (source: boolean) => {
        invoke('go_dir_back', { source }).then(() => {
            loadFiles(source);
        });
    }

    onMount(() => {
        loadFiles(true);
        loadFiles(false);
    });
</script>

<div class="flex flex-wrap items-center px-5 py-1">
    <p class="mr-3">Choose algorithm:</p>
    <Select.Root type="single" bind:value={algo}>
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
                        disabled={algo === cipher.value} 
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
                        disabled={algo === cipher.value} 
                        class="hover:text-fg-0 hover:bg-bg-3/50"
                    >
                        {cipher.label}
                    </Select.Item>
                {/each}
            </Select.Group>
        </Select.Content>
    </Select.Root>
    {#if algo.startsWith("block:")}
        <p class="mx-3">Choose mode:</p>
        <Select.Root type="single" bind:value={mode}>
            <Select.Trigger class="border-bg-5 data-[placeholder]:text-fg-3 min-w-40">
                {triggerContentMode}
            </Select.Trigger>
            <Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
                <Select.Group>
                    {#each blockModes as blockMode}
                        <Select.Item 
                            value={blockMode.value}
                            label={blockMode.label} 
                            disabled={mode === blockMode.value} 
                            class="hover:text-fg-0 hover:bg-bg-3/50"
                        >
                            {blockMode.label}
                        </Select.Item>
                    {/each}
                </Select.Group>
            </Select.Content>
        </Select.Root>
    {/if}
    <p class="ml-auto mr-2">Operation:</p>
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
            onGoBack={() => goDirBack(true) }
            onChangeDir={dir => changeDir(dir, true)}
            onFileAction={() => {}}
            onSetAbsolutePath={() => true}
            onLockChange={() => true}
            onRefresh={() => {}}
        />
    </div>
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer 
            class="" 
            pwd={destCwd} 
            files={destFiles} 
            label="Destination Directory" 
            bind:locked={destLocked}
            lockIcon={LockIcon}
            unlockIcon={LockOpenIcon}
            onGoBack={() => goDirBack(false) }
            onChangeDir={dir => changeDir(dir, false)}
            onFileAction={() => {}}
            onSetAbsolutePath={() => false}
            onLockChange={() => false}
            onRefresh={() => {}}
        />
    </div>
</div>