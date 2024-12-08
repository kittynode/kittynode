<script lang="ts">
import { onMount } from "svelte";
import { remoteAccessStore } from "$stores/remoteAccess.svelte";
import { systemInfoStore } from "$stores/systemInfo.svelte";
import { Skeleton } from "$lib/components/ui/skeleton";
import { Progress } from "$lib/components/ui/progress";
import * as Card from "$lib/components/ui/card";
import { formatBytes, calculateUsagePercentage } from "$lib/utils/format";

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

<ul class="mb-8 list-disc pl-4 space-y-2">
  <li>Remote access: <strong>{remoteAccessStore.remoteAccess ? "Enabled" : "Not enabled"}</strong></li>
</ul>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  System Info
</h3>

{#if systemInfoStore.systemInfo}
<div class="space-y-4">
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <Card.Root>
      <Card.Header>
        <Card.Title>Processor</Card.Title>
      </Card.Header>
      <Card.Content>
        {systemInfoStore.systemInfo.processor.name}
        ({systemInfoStore.systemInfo.processor.cores} cores,
         {systemInfoStore.systemInfo.processor.frequency_ghz.toFixed(2)} GHz,
         {systemInfoStore.systemInfo.processor.architecture})
      </Card.Content>
    </Card.Root>

    <Card.Root>
      <Card.Header>
        <Card.Title>Memory</Card.Title>
      </Card.Header>
      <Card.Content>
        {formatBytes(systemInfoStore.systemInfo.memory.total_bytes)}
      </Card.Content>
    </Card.Root>
  </div>

  <Card.Root>
    <Card.Header>
      <Card.Title>Storage</Card.Title>
    </Card.Header>
    <Card.Content class="space-y-4">
      {#each systemInfoStore.systemInfo.storage.disks as disk}
        <div>
          <div class="text-sm font-medium mb-2">
            {disk.name} ({disk.mount_point})
          </div>
          <Progress
            value={calculateUsagePercentage(
              disk.total_bytes - disk.available_bytes,
              disk.total_bytes
            )}
            max={100}
          />
          <div class="flex justify-between text-sm mt-2">
            <span>
              Used: {calculateUsagePercentage(
                disk.total_bytes - disk.available_bytes,
                disk.total_bytes
              )}%
            </span>
            <span>
              {formatBytes(disk.available_bytes)} available /
              {formatBytes(disk.total_bytes)} total
            </span>
          </div>
        </div>
      {/each}
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
