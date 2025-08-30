<script lang="ts">
    import Palette from '@lucide/svelte/icons/palette';
    import { THEMES, updateThemeInHtml, type Theme } from '$lib/theme';
    import type { ChangeEventHandler } from 'svelte/elements';
    import { theme } from '$lib';

    const switchTheme: ChangeEventHandler<HTMLSelectElement> = (e) => {
        const theme: Theme = e.currentTarget!.value as Theme;
        updateThemeInHtml(theme);
    };
</script>

<div class="input-group hover:preset-tonal grid-cols-[auto_1fr] py-1">
    <div class="ig-cell">
        <Palette size={18} />
    </div>
    <select
        class="ig-select appearance-none px-4 capitalize outline-0"
        bind:value={theme.current}
        onchange={switchTheme}
    >
        {#each THEMES as theme}
            <option value={theme} class="!border-none capitalize">{theme}</option>
        {/each}
    </select>
</div>
