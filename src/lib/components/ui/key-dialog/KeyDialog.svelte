<script lang="ts">
	import { KeySquareIcon, XIcon } from "@lucide/svelte";
	import { Dialog, Separator } from "bits-ui";


</script>

<Dialog.Root>
    <Dialog.Trigger>
        <KeySquareIcon class="bg-bg-3 hover:bg-bg-4 rounded-md text-primary p-2 size-10 mr-5 cursor-pointer" />
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
            <Dialog.Close
                class="absolute right-2 top-2 text-fg-3 cursor-pointer"
            >
                <XIcon class="size-6"/>
            </Dialog.Close>
        </Dialog.Content>
    </Dialog.Portal>
</Dialog.Root>