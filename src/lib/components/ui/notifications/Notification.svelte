<script lang="ts">
    import { fly, fade } from "svelte/transition";
    import { removeNotification, type Notification, type NotificationType } from "./store";
	import { CircleAlertIcon, CircleCheckIcon, CircleXIcon, InfoIcon, XIcon } from "@lucide/svelte";

    const { notification }: { notification: Notification } = $props();

    const colors: Record<NotificationType, string> = {
        info: "text-primary",
        success: "text-success",
        warning: "text-warning",
        error: "text-error"
    };

    const Icons = {
        info: InfoIcon,
        warning: CircleAlertIcon,
        error: CircleXIcon,
        success: CircleCheckIcon
    };
</script>

<div
    class={`flex items-center justify-between p-4 rounded shadow-md w-100 min-h-20 text-white bg-bg-3 border border-fg-5/50 pointer-events-auto`}
    in:fly={{ y: -20, duration: 300 }}
    out:fade={{ duration: 300 }}
>
    <!-- svelte-ignore svelte_component_deprecated -->
    <svelte:component this={Icons[notification.type]} class={`${colors[notification.type]} flex-shrink-0 size-7`} />
    <div class="mx-3 text-wrap text-center">{notification.message}</div>
    <button
        class="ml-4 font-bold outline-none cursor-pointer flex-shrink-0"
        onclick={(e) => {
            e.stopPropagation(); removeNotification(notification?.id ?? -1);
        }}
    >
        <XIcon />
    </button>
</div>