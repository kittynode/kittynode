<script lang="ts">
import { onMount } from "svelte"; // Use this to run when component mounts
import { invoke } from "@tauri-apps/api/core";
import type { SystemInfo } from "$lib/types/system_info";

// State variables to store system information
let processor = $state("Loading...");
let memory = $state("Loading...");
let storage = $state("Loading...");

// Function to fetch system info from the backend
async function fetchSystemInfo() {
  try {
    const systemInfo: SystemInfo = await invoke("system_info");
    // Extract information from the systemInfo object and assign it to the state variables
    processor = systemInfo.processor;
    memory = systemInfo.memory;
    storage = systemInfo.storage;
  } catch (error) {
    console.error("Failed to fetch system info:", error);
  }
}

// Fetch system info when the component mounts
onMount(() => {
  fetchSystemInfo();
});
</script>

<ul>
  <li>Processor: {processor}</li>
  <li>Memory: {memory}</li>
  <li>Storage: {storage}</li>
</ul>
