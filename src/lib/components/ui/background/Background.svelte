<script lang="ts">
	import { LockKeyhole} from '@lucide/svelte';
</script>

<div class="bg-bg-0 fixed w-full h-full flex items-center -z-999 transition-colors duration-200">
	<LockKeyhole class="absolute rotate-20 right-[20%] translate-x-[50%] size-80 text-bg-2 opacity-80" />
	<LockKeyhole class="absolute -rotate-30 left-[20%] translate-x-[-50%] -translate-y-10 size-90 text-bg-2 opacity-60"/>
	<LockKeyhole class="absolute rotate-80 left-[60%] top-[10%] size-40 text-bg-1" />
	<LockKeyhole class="absolute rotate-130 left-[30%] top-[10%] size-30 text-bg-2 opacity-40" />
	<LockKeyhole class="absolute rotate-10 left-[50%] translate-x-[-50%] bottom-5 size-50 text-bg-2 opacity-50" />
</div>