<script lang="ts">
import { onMount } from "svelte";
import { remoteAccessStore } from "$stores/remoteAccess.svelte";
import { systemInfoStore } from "$stores/systemInfo.svelte";
import { Skeleton } from "$lib/components/ui/skeleton";

function fetchSystemInfo() {
  systemInfoStore.fetchSystemInfo();
}

onMount(() => {
  if (!systemInfoStore.systemInfo) {
    fetchSystemInfo();
  }
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

{#if systemInfoStore.systemInfo}
<ul>
  <li>Processor: {systemInfoStore.systemInfo.processor}</li>
  <li>Memory: {systemInfoStore.systemInfo.memory}</li>
  <li>Storage: {systemInfoStore.systemInfo.storage}</li>
</ul>
{:else}
<div class="space-y-2">
  <Skeleton class="h-4 w-[250px]" />
  <Skeleton class="h-4 w-[250px]" />
  <Skeleton class="h-4 w-[250px]" />
</div>
{/if}
