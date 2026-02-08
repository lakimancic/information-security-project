<script lang="ts">
	import { LockKeyhole } from '@lucide/svelte';
</script>

<div
	class="bg-bg-0 fixed -z-999 flex h-full w-full items-center transition-colors duration-200"
>
	<LockKeyhole
		class="text-bg-2 absolute right-[20%] size-80 translate-x-[50%] rotate-20 opacity-80"
	/>
	<LockKeyhole
		class="text-bg-2 absolute left-[20%] size-90 translate-x-[-50%] -translate-y-10 -rotate-30 opacity-60"
	/>
	<LockKeyhole
		class="text-bg-1 absolute top-[10%] left-[60%] size-40 rotate-80"
	/>
	<LockKeyhole
		class="text-bg-2 absolute top-[10%] left-[30%] size-30 rotate-130 opacity-40"
	/>
	<LockKeyhole
		class="text-bg-2 absolute bottom-5 left-[50%] size-50 translate-x-[-50%] rotate-10 opacity-50"
	/>
</div>
