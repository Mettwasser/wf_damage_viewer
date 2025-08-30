<script lang="ts">
    import '../app.css';
    import ThemeButton from '../components/ThemeButton.svelte';
    import { initTheme } from '../lib/theme';
    import { AppBar, Toaster } from '@skeletonlabs/skeleton-svelte';
    import { toaster } from '$lib';

    let { children } = $props();

    initTheme();
</script>

<div class="flex w-full flex-1 flex-col">
    <div class="flex h-auto w-full flex-col">
        <AppBar classes="top-0" leadClasses="flex items-center">
            {#snippet lead()}
                <h1 class="h4">Warframe Damage Viewer</h1>
            {/snippet}

            {#snippet trail()}
                <ThemeButton />
            {/snippet}
        </AppBar>

        <Toaster {toaster}></Toaster>
    </div>

    <div class="flex h-full flex-col items-center justify-center">
        {@render children()}
    </div>
</div>
