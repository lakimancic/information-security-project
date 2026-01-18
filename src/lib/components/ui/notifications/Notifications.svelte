<script lang="ts">
    import { notifications } from "./store";
    import Notification from "./Notification.svelte";
</script>

<div class="fixed top-5 left-1/2 transform -translate-x-1/2 flex flex-col gap-2 z-50 pointer-events-none">
    {#each $notifications as notif (notif.id)}
        <Notification notification={notif} />
    {/each}
</div>