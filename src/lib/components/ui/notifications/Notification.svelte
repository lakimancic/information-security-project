<script lang="ts">
	import { fly, fade } from 'svelte/transition';
	import {
		removeNotification,
		type Notification,
		type NotificationType
	} from './store';
	import {
		CircleAlertIcon,
		CircleCheckIcon,
		CircleXIcon,
		InfoIcon,
		XIcon
	} from '@lucide/svelte';

	const { notification }: { notification: Notification } = $props();

	const colors: Record<NotificationType, string> = {
		info: 'text-primary',
		success: 'text-success',
		warning: 'text-warning',
		error: 'text-error'
	};

	const Icons = {
		info: InfoIcon,
		warning: CircleAlertIcon,
		error: CircleXIcon,
		success: CircleCheckIcon
	};
</script>

<div
	class={`bg-bg-3 border-fg-5/50 pointer-events-auto flex min-h-20 w-100 items-center justify-between rounded border p-4 text-white shadow-md`}
	in:fly={{ y: -20, duration: 300 }}
	out:fade={{ duration: 300 }}
>
	<!-- svelte-ignore svelte_component_deprecated -->
	<svelte:component
		this={Icons[notification.type]}
		class={`${colors[notification.type]} size-7 flex-shrink-0`}
	/>
	<div class="mx-3 text-center text-wrap">{notification.message}</div>
	<button
		class="ml-4 flex-shrink-0 cursor-pointer font-bold outline-none"
		onclick={(e) => {
			e.stopPropagation();
			removeNotification(notification?.id ?? -1);
		}}
	>
		<XIcon />
	</button>
</div>
