<script lang="ts">
import { onMount, onDestroy } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import { serverUrlStore } from "$stores/serverUrl.svelte";

let {
  containerName,
  tailLines,
}: { containerName: string; tailLines: number | null } = $props();

let logs: string[] = $state([]);
let logsElement: HTMLDivElement;
let shouldAutoScroll = $state(true);
let polling: NodeJS.Timeout;

async function fetchLogs() {
  try {
    const newLogs = await invoke<string[]>("get_container_logs", {
      containerName,
      tailLines,
      serverUrl: serverUrlStore.serverUrl,
    });

    logs = newLogs;

    // Schedule scroll after render if we should auto scroll
    if (shouldAutoScroll) {
      queueMicrotask(() => {
        if (logsElement) {
          logsElement.scrollTop = logsElement.scrollHeight;
        }
      });
    }
  } catch (error) {
    console.error("Failed to fetch logs:", error);
  }
}

function startPolling() {
  polling = setInterval(fetchLogs, 2000);
}

onMount(() => {
  fetchLogs();
  startPolling();
});

onDestroy(() => {
  if (polling) clearInterval(polling);
});

function handleScroll(e: Event) {
  const target = e.target as HTMLDivElement;
  const isAtBottom =
    Math.abs(target.scrollHeight - target.clientHeight - target.scrollTop) < 1;
  shouldAutoScroll = isAtBottom;
}
</script>

<div class="flex flex-col gap-2">
    <div
        bind:this={logsElement}
        onscroll={handleScroll}
        class="h-[400px] overflow-y-auto overflow-x-hidden bg-black/90 text-white p-4 font-mono text-sm rounded"
    >
        {#each logs as log}
            <div class="whitespace-pre-line break-words">
                {log}
            </div>
        {/each}

        {#if logs.length === 0}
            <div class="text-gray-400">No logs available</div>
        {/if}
    </div>
</div>
