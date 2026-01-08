<script lang="ts">
	import { KeySquareIcon, XIcon } from "@lucide/svelte";
	import { Dialog, Separator, Tabs } from "bits-ui";
    import * as Select from "$lib/components/ui/select/index";
	import type { CipherTag, Key } from "$lib/types/crypto";

    let {
        algo,
        mode,
        outputKey = $bindable(null)
    } : {
        algo: CipherTag|null,
        mode: CipherTag|null,
        outputKey: Key|null,
    } = $props();

    const keyNames = [
        'hello world',
        'world hello',
        'alpha key',
    ];

    let selectedKey = $state(outputKey?.label ?? '');
    let errorMsg = $state('');

    const triggerContentKey = $derived(
        keyNames.find(m => m === selectedKey) ?? "encryption_key_name"
    );
</script>

<Dialog.Root>
    <Dialog.Trigger 
        disabled={algo === null || (algo.value.startsWith("block") && mode === null)}
        class={algo === null || (algo.value.startsWith("block") && mode === null) ? "text-fg-3/40" : "text-primary"}
    >
        <KeySquareIcon class="bg-bg-3 hover:bg-bg-4 rounded-md p-2 size-10 mr-5 cursor-pointer" />
    </Dialog.Trigger>
    <Dialog.Portal>
        <Dialog.Overlay
            class="data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 fixed inset-0 z-50 bg-bg-0/80"
        />
        <Dialog.Content
            class="bg-bg-1 text-fg-0 border border-bg-4 rounded-md data-[state=open]:animate-in data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 outline-hidden fixed left-[50%] top-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] p-5 sm:max-w-[490px] md:w-full"
        >
            <Dialog.Title
                class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
            >
                Encryption Key Handler
            </Dialog.Title>
            <Separator.Root class="bg-bg-4 -mx-5 mb-6 mt-5 block h-px" />
            <Dialog.Description class="text-fg-3 text-sm">
                Create and manage your keys for file encryption and decryption. You can create or manage multiple keys to many different ciphers.
            </Dialog.Description>
            <div class="my-3 text-xl grid grid-cols-2">
                <p>Algorithm: <span class="text-primary font-black">{algo?.label ?? ''}</span></p>
            {#if algo?.value.startsWith("block")}
                <p>Block Mode: <span class="text-primary font-black">{mode?.label ?? ''}</span></p>
            {/if}
            </div>
            <Tabs.Root
                value="select-existing"
                class="mt-4"
            >
                <Tabs.List
                    class="rounded-md border border-bg-3 bg-bg-1 grid w-full grid-cols-2 gap-1 p-1"
                >
                    <Tabs.Trigger 
                        value="select-existing"
                        class="data-[state=active]:bg-bg-0 rounded-md"
                    >Select Existing</Tabs.Trigger>
                    <Tabs.Trigger 
                        value="generate-new"
                        class="data-[state=active]:bg-bg-0 rounded-md"
                    >Generate New</Tabs.Trigger>
                </Tabs.List>
                <Tabs.Content value="select-existing" class="py-3">
                    <p>Choose existing key:</p>
                    <Select.Root type="single" bind:value={selectedKey}>
                        <Select.Trigger class="border-bg-5 data-[placeholder]:text-fg-0/40 min-w-40 text-md p-5 w-full mt-2 mb-4">
                            {triggerContentKey}
                        </Select.Trigger>
                        <Select.Content class="bg-bg-2 text-fg-1 border-bg-4">
                            <Select.Group>
                                {#each keyNames as keyName}
                                    <Select.Item 
                                        value={keyName} 
                                        label={keyName}
                                        class="hover:text-fg-0 hover:bg-bg-3/50 text-md"
                                    >
                                    {keyName}</Select.Item>
                                {/each}
                            </Select.Group>
                        </Select.Content>
                    </Select.Root>
                    <p>Enter password:</p>
                    <input 
                        type="password" 
                        name="key_password" 
                        id="keyPassword"
                        placeholder="key_password"
                        class="outline-none border border-bg-5 min-w-40 text-md py-2 px-3 rounded-md w-full mt-2 mb-4 placeholder:text-fg-0/40"
                    />
                    <p class="text-center text-error">{errorMsg}</p>
                    <div class="flex w-full justify-end mt-5">
                        <Dialog.Close
                            class="bg-primary px-4 py-3 font-semibold rounded-md cursor-pointer hover:bg-primary/70 transition-colors duration-300"
                        >Select Key</Dialog.Close>
                    </div>
                </Tabs.Content>
                <Tabs.Content value="generate-new" class="py-3">
                    <p>Enter new key name:</p>
                    <input 
                        type="text" 
                        name="key_name" 
                        id="keyName"
                        placeholder="key_name"
                        class="outline-none border border-bg-5 min-w-40 text-md py-2 px-3 rounded-md w-full mt-2 mb-4 placeholder:text-fg-0/40"
                    />
                    <p>Enter key password:</p>
                    <input 
                        type="password" 
                        name="key_password" 
                        id="keyPassword"
                        placeholder="key_password"
                        class="outline-none border border-bg-5 min-w-40 text-md py-2 px-3 rounded-md w-full mt-2 mb-4 placeholder:text-fg-0/40"
                    />
                    <p>Confirm key password:</p>
                    <input 
                        type="password" 
                        name="confirm_password"
                        id="confirmPassword"
                        placeholder="confirm_password"
                        class="outline-none border border-bg-5 min-w-40 text-md py-2 px-3 rounded-md w-full mt-2 mb-4 placeholder:text-fg-0/40"
                    />
                    <p class="text-center text-error">{errorMsg}</p>
                    <div class="flex w-full justify-end mt-5">
                        <Dialog.Close
                            class="bg-primary px-4 py-3 font-semibold rounded-md cursor-pointer hover:bg-primary/70 transition-colors duration-300"
                        >Generate Key</Dialog.Close>
                    </div>
                </Tabs.Content>
            </Tabs.Root>
            <Dialog.Close
                class="absolute right-2 top-2 text-fg-3 cursor-pointer"
            >
                <XIcon class="size-6"/>
            </Dialog.Close>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>