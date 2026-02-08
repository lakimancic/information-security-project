<script lang="ts">
	import { notifications } from './store';
	import Notification from './Notification.svelte';
</script>

<div
	class="pointer-events-none fixed top-5 left-1/2 z-50 flex -translate-x-1/2 transform flex-col gap-2"
>
	{#each $notifications as notif (notif.id)}
		<Notification notification={notif} />
	{/each}
</div>
