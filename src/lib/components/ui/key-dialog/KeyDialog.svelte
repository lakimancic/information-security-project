<script lang="ts">
	import { KeySquareIcon, XIcon } from '@lucide/svelte';
	import { Dialog, Separator, Tabs } from 'bits-ui';
	import * as Select from '$lib/components/ui/select/index';
	import type { CipherTag, Key, ShortKey } from '$lib/types/crypto';
	import { invoke } from '@tauri-apps/api/core';

	let {
		algo,
		mode,
		operation,
		outputKey = $bindable(null),
		onKeySet
	}: {
		algo: CipherTag | null;
		mode: CipherTag | null;
		operation: 'dec' | 'enc';
		outputKey: Key | null;
		onKeySet: (key: Key) => void;
	} = $props();

	let keyNames = $state<string[]>([]);

	let selectedKey = $state(outputKey?.label ?? '');
	let selectedPass = $state('');

	let errorMsg = $state('');
	let dialogOpen = $state(false);

	let genName = $state('');
	let genPass = $state('');
	let genPassRep = $state('');

	let currentTab = $state('select-existing');

	const triggerContentKey = $derived(
		keyNames.find((m) => m === selectedKey) ?? 'encryption_key_name'
	);

	const handleDialogOpen = async (value: boolean) => {
		if (value) {
			errorMsg = '';
			genName = '';
			genPass = '';
			genPassRep = '';
			selectedKey = outputKey?.label ?? '';
			selectedPass = '';
			currentTab = 'select-existing';

			if (operation === 'enc') {
				invoke('find_keys_by_algo', {
					algorithm: algo?.value ?? '',
					mode: algo?.value
				}).then((res) => {
					keyNames = res as string[];
					dialogOpen = value;
				});
			} else {
				invoke('list_keys').then((res) => {
					keyNames = (res as ShortKey[]).map((sk) => sk.name);
					dialogOpen = value;
				});
			}
		} else {
			dialogOpen = value;
		}
	};

	const handleGenerateKey = async () => {
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
			algorithm: algo?.value ?? '',
			mode: mode?.value,
			name: genName,
			password: genPass
		})
			.then((res: any) => {
				outputKey = {
					...res,
					label: genName
				};
				dialogOpen = false;
				if (outputKey) {
					onKeySet(outputKey);
				}
			})
			.catch((err: any) => {
				errorMsg = err.message;
			});
	};

	const handleChangeKey = async () => {
		invoke('find_key', { name: selectedKey, password: selectedPass })
			.then((res: any) => {
				outputKey = {
					...res,
					label: selectedKey
				};
				dialogOpen = false;
				if (outputKey) {
					onKeySet(outputKey);
				}
			})
			.catch((err: any) => {
				errorMsg = err.message;
			});
	};
</script>

<Dialog.Root
	bind:open={dialogOpen}
	onOpenChange={(value) => handleDialogOpen(value)}
>
	<Dialog.Trigger
		disabled={(algo === null ||
			(algo.value.startsWith('block') && mode === null)) &&
			operation === 'enc'}
		class={(algo === null ||
			(algo.value.startsWith('block') && mode === null)) &&
		operation === 'enc'
			? 'text-fg-3/40'
			: 'text-primary'}
	>
		<KeySquareIcon
			class="bg-bg-3 hover:bg-bg-4 mr-5 size-10 cursor-pointer rounded-md p-2"
		/>
	</Dialog.Trigger>
	<Dialog.Portal>
		<Dialog.Overlay
			class="data-[state=open]:animate-in data-[state=closed]:animate-out 
            data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 
            bg-bg-0/80 fixed inset-0 z-50"
		/>
		<Dialog.Content
			class="bg-bg-1 text-fg-0 border-bg-4 data-[state=open]:animate-in 
            data-[state=closed]:animate-out data-[state=closed]:fade-out-0 data-[state=open]:fade-in-0 
            data-[state=closed]:zoom-out-95 data-[state=open]:zoom-in-95 fixed top-[50%] left-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] 
            rounded-md border p-5 outline-hidden sm:max-w-[490px] md:w-full"
		>
			<Dialog.Title
				class="flex w-full items-center justify-center text-lg font-semibold tracking-tight"
			>
				Encryption Key Handler
			</Dialog.Title>
			<Separator.Root class="bg-bg-4 -mx-5 mt-5 mb-6 block h-px" />
			<Dialog.Description class="text-fg-3 text-sm">
				Create and manage your keys for file encryption and decryption. You can
				create or manage multiple keys to many different ciphers.
			</Dialog.Description>
			{#if operation === 'enc'}
				<div class="my-3 grid grid-cols-2 text-xl">
					<p>
						Algorithm: <span class="text-primary font-black"
							>{algo?.label ?? ''}</span
						>
					</p>
					{#if algo?.value.startsWith('block')}
						<p>
							Block Mode: <span class="text-primary font-black"
								>{mode?.label ?? ''}</span
							>
						</p>
					{/if}
				</div>
			{/if}
			<Tabs.Root bind:value={currentTab} class="mt-4">
				<Tabs.List
					class="border-bg-3 bg-bg-1 grid w-full grid-cols-2 gap-1 rounded-md border p-1"
				>
					<Tabs.Trigger
						value="select-existing"
						class="data-[state=active]:bg-bg-0 rounded-md"
						>Select Existing</Tabs.Trigger
					>
					<Tabs.Trigger
						value="generate-new"
						class="data-[state=active]:bg-bg-0 disabled:text-fg-1/50 rounded-md"
						disabled={operation === 'dec'}>Generate New</Tabs.Trigger
					>
				</Tabs.List>
				<Tabs.Content value="select-existing" class="py-3">
					<p>Choose existing key:</p>
					<Select.Root type="single" bind:value={selectedKey}>
						<Select.Trigger
							class="border-bg-5 data-[placeholder]:text-fg-0/40 text-md mt-2 mb-4 w-full min-w-40 p-5"
						>
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
										{keyName}</Select.Item
									>
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
						class="border-bg-5 text-md placeholder:text-fg-0/40 mt-2 mb-4 w-full min-w-40 rounded-md border px-3 py-2 outline-none"
						bind:value={selectedPass}
					/>
					<p class="text-error text-center">{errorMsg}</p>
					<div class="mt-5 flex w-full justify-end">
						<button
							onclick={handleChangeKey}
							class="bg-primary hover:bg-primary/70 cursor-pointer rounded-md px-4 py-3 font-semibold transition-colors duration-300"
							>Select Key</button
						>
					</div>
				</Tabs.Content>
				<Tabs.Content value="generate-new" class="py-3">
					<p>Enter new key name:</p>
					<input
						type="text"
						name="key_name"
						id="keyName"
						placeholder="key_name"
						class="border-bg-5 text-md placeholder:text-fg-0/40 mt-2 mb-4 w-full min-w-40 rounded-md border px-3 py-2 outline-none"
						bind:value={genName}
					/>
					<p>Enter key password:</p>
					<input
						type="password"
						name="key_password"
						id="keyPassword"
						placeholder="key_password"
						class="border-bg-5 text-md placeholder:text-fg-0/40 mt-2 mb-4 w-full min-w-40 rounded-md border px-3 py-2 outline-none"
						bind:value={genPass}
					/>
					<p>Confirm key password:</p>
					<input
						type="password"
						name="confirm_password"
						id="confirmPassword"
						placeholder="confirm_password"
						class="border-bg-5 text-md placeholder:text-fg-0/40 mt-2 mb-4 w-full min-w-40 rounded-md border px-3 py-2 outline-none"
						bind:value={genPassRep}
					/>
					<p class="text-error text-center">{errorMsg}</p>
					<div class="mt-5 flex w-full justify-end">
						<button
							class="bg-primary hover:bg-primary/70 cursor-pointer rounded-md px-4 py-3 font-semibold transition-colors duration-300"
							onclick={handleGenerateKey}>Generate Key</button
						>
					</div>
				</Tabs.Content>
			</Tabs.Root>
			<Dialog.Close class="text-fg-3 absolute top-2 right-2 cursor-pointer">
				<XIcon class="size-6" />
			</Dialog.Close>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
