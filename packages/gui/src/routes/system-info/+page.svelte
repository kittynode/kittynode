<script lang="ts">
import { onMount } from "svelte";
import { remoteAccessStore } from "$stores/remoteAccess.svelte";
import { systemInfoStore } from "$stores/systemInfo.svelte";
import { Skeleton } from "$lib/components/ui/skeleton";
import { Progress } from "$lib/components/ui/progress";
import * as Card from "$lib/components/ui/card";

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
  System Info
</h3>

{#if systemInfoStore.systemInfo}
<div class="space-y-4">
  <Card.Root>
    <Card.Header>
      <Card.Title>Processor</Card.Title>
    </Card.Header>
    <Card.Content>{systemInfoStore.systemInfo.processor}</Card.Content>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title>Memory</Card.Title>
    </Card.Header>
    <Card.Content>{systemInfoStore.systemInfo.memory}</Card.Content>
  </Card.Root>

  <Card.Root>
    <Card.Header>
      <Card.Title>Storage</Card.Title>
    </Card.Header>
    <Card.Content>
      <Progress value={systemInfoStore.systemInfo.storage_percentage} max={100} />
      <div class="flex justify-between mt-2">
        <span>Used: {systemInfoStore.systemInfo.storage_percentage.toFixed(2)}%</span>
        <span>{systemInfoStore.systemInfo.storage}</span>
      </div>
    </Card.Content>
  </Card.Root>
</div>
{:else}
<div class="space-y-2">
  <Skeleton class="h-4 w-[250px]" />
  <Skeleton class="h-4 w-[250px]" />
  <Skeleton class="h-4 w-[250px]" />
</div>
{/if}
