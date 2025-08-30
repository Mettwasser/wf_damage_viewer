<script lang="ts">
    import { Channel } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';

    let container: HTMLDivElement;

    let logs: string[] = $state([]);
    let isRunning = $state(false);

    listen<string>('dmg-too-high', (event) => {
        logs.push(event.payload);
    });

    $effect(() => {
        logs.length;

        if (container) {
            container.scrollTop = container.scrollHeight;
        }
    });

    function toggleIsRunning() {
        isRunning = !isRunning;
        logs.push(`Turned Log Listener ${isRunning ? 'ON' : 'OFF'}`);
    }
</script>

<div class="flex w-2/3 flex-col gap-4">
    <div>
        <button
            class="btn preset-filled-error w-20"
            class:preset-filled-error-200-800={isRunning}
            class:preset-filled-success-200-800={!isRunning}
            onclick={toggleIsRunning}
        >
            {isRunning ? 'Stop' : 'Start'}
        </button>
    </div>
    <div
        bind:this={container}
        class="bg-surface-700 h-[50dvh] w-full overflow-auto scroll-auto p-4"
    >
        {#each logs as log}
            <p>{log}</p>
            <br />
        {/each}
    </div>
</div>
