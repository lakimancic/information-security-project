<script lang="ts">
	import { Moon, Sun } from '@lucide/svelte';
	import { onMount } from 'svelte';

	let { className = '' } = $props();

	let isDark = $state(false);

	onMount(() => {
		const savedTheme = localStorage.getItem('theme');

		if (savedTheme) {
			isDark = savedTheme === 'dark';
		} else {
			isDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
		}

		applyTheme();

		const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
		const handleChange = (e: MediaQueryListEvent) => {
			if (!localStorage.getItem('theme')) {
				isDark = e.matches;
				applyTheme();
			}
		};

		mediaQuery.addEventListener('change', handleChange);
		return () => mediaQuery.removeEventListener('change', handleChange);
	});

	function toggleTheme() {
		isDark = !isDark;
		localStorage.setItem('theme', isDark ? 'dark' : 'light');
		applyTheme();
	}

	function applyTheme() {
		if (isDark) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}
</script>

<button
	onclick={toggleTheme}
	class="inline-flex items-center justify-center cursor-pointer bg-transparent border-none p-2 rounded-md transition-all duration-200 hover:bg-black/5 dark:hover:bg-white/10 {className}"
	aria-label={isDark ? 'Switch to light theme' : 'Switch to dark theme'}
>
	{#if isDark}
		<Sun class="transition-all duration-300 hover:rotate-12 hover:scale-110" />
	{:else}
		<Moon class="transition-all duration-300 hover:rotate-12 hover:scale-110" />
	{/if}
</button>