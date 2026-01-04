<script lang="ts">
	import FileExplorer from "$lib/components/ui/file-explorer/FileExplorer.svelte";
	import type { LocalFile } from "$lib/components/ui/file-explorer/utils";
	import * as Select from "$lib/components/ui/select/index";

    const files : LocalFile[] = [
        { filename: 'Hello Wolrd', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'File Folder', type: 'folder'},
        { filename: 'document.txt', lastModified: '2024-06-01 10:00', size: 1000, typeLong: 'Text File', type: 'text'},
        { filename: 'image.png', lastModified: '2024-05-28 14:30', size: 1000, typeLong: 'Image File', type: 'image'},
        { filename: 'presentation.pptx', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Presentation File', type: 'ppt'},
        { filename: 'document.docx', lastModified: '2024-05-20 09:15', size: 100050, typeLong: 'Document File', type: 'doc'},
        { filename: 'sheet.xls', lastModified: '2024-05-20 09:15', size: 1000, typeLong: 'Spreadsheet File', type: 'xls'},
        { filename: 'archive.zip', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Compressed Folder', type: 'zip' },
        { filename: 'info.pdf', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'PDF Document', type: 'pdf'},
        { filename: 'game.exe', lastModified: '2024-04-15 16:45', size: 1000, typeLong: 'Executable File', type: 'exe'}
    ];

    let algo = $state('');
    let mode = $state('');
    let operation = $state<'dec'|'enc'>('enc');

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
</script>

<div class="flex items-center px-5 py-1">
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
    >{operation === 'enc' ? 'Encryption' : 'Decryption'}</button>
</div>
<div class="flex flex-1 min-h-0">
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer class="" pwd="/home/lazarm" files={files} label="Source Directory" />
    </div>
    <div class="flex-1 p-4 min-h-0 overflow-hidden">
        <FileExplorer class="" pwd="/home/lazarm" files={files} label="Destination Directory" />
    </div>
</div>