<script lang="ts">
import { onMount } from "svelte";
import type { SystemInfo } from "$lib/types/system_info";
import { invoke } from "@tauri-apps/api/core";

let processor = $state("Loading...");
let memory = $state("Loading...");
let storage = $state("Loading...");

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

onMount(() => {
  fetchSystemInfo();
});
</script>

<h2>System information</h2>
<ul>
  <li>Processor: {processor}</li>
  <li>Memory: {memory}</li>
  <li>Storage: {storage}</li>
</ul>
