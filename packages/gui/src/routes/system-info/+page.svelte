<script lang="ts">
import { onMount } from "svelte";
import type { SystemInfo } from "$lib/types/system_info";
import { invoke } from "@tauri-apps/api/core";
import { platform } from "@tauri-apps/plugin-os";

let processor = $state("Loading...");
let memory = $state("Loading...");
let storage = $state("Loading...");
let capabilities: string[] = $state([]);

async function fetchSystemInfo() {
  try {
    const systemInfo: SystemInfo = await invoke("system_info");
    processor = systemInfo.processor;
    memory = systemInfo.memory;
    storage = systemInfo.storage;
  } catch (error) {
    console.error("Failed to fetch system info:", error);
  }
}

onMount(async () => {
  if (!["ios", "android"].includes(platform())) {
    await fetchSystemInfo();
  }
  capabilities = await invoke("get_capabilities");
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Capabilities
</h3>

<ul class="mb-8">
  <li>Remote control: <strong>{capabilities.includes("remote_control") ? "Enabled" : "Not enabled"}</strong></li>
</ul>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  System info
</h3>

{#if ["ios", "android"].includes(platform())}
  <p>This feature is not yet supported on mobile.</p>
{:else}
  <ul>
    <li>Processor: {processor}</li>
    <li>Memory: {memory}</li>
    <li>Storage: {storage}</li>
  </ul>
{/if}
