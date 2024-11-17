<script lang="ts">
import { onMount } from "svelte";
import type { SystemInfo } from "$lib/types/system_info";
import { invoke } from "@tauri-apps/api/core";
import { remoteAccessStore } from "$stores/remoteAccess.svelte";
import { serverUrlStore } from "$stores/serverUrl.svelte";

let processor = $state("Loading...");
let memory = $state("Loading...");
let storage = $state("Loading...");

async function fetchSystemInfo() {
  try {
    const systemInfo: SystemInfo = await invoke("system_info", {
      serverUrl: serverUrlStore.serverUrl,
    });
    processor = systemInfo.processor;
    memory = systemInfo.memory;
    storage = systemInfo.storage;
  } catch (error) {
    console.error("Failed to fetch system info:", error);
  }
}

onMount(async () => {
  await fetchSystemInfo();
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Capabilities
</h3>

<ul class="mb-8">
  <li>Remote access: <strong>{remoteAccessStore.remoteAccess ? "Enabled" : "Not enabled"}</strong></li>
</ul>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  System info
</h3>

<ul>
  <li>Processor: {processor}</li>
  <li>Memory: {memory}</li>
  <li>Storage: {storage}</li>
</ul>
