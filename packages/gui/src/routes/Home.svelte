<script lang="ts">
import { onMount } from "svelte";
import type { Package } from "$lib/types";
import { Button } from "$lib/components/ui/button";
import * as Card from "$lib/components/ui/card";
import { platform } from "@tauri-apps/plugin-os";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { systemInfoStore } from "$stores/systemInfo.svelte";
import { selectedPackageStore } from "$stores/selectedPackage.svelte";
import { packagesStore } from "$stores/packages.svelte";
import { goto } from "$app/navigation";

function selectPackage(pkg: Package) {
  selectedPackageStore.setPackage(pkg);
  goto("/package");
}

function isMobileAndLocal() {
  return (
    ["ios", "android"].includes(platform()) && serverUrlStore.serverUrl === ""
  );
}

onMount(async () => {
  if (!systemInfoStore.systemInfo) systemInfoStore.fetchSystemInfo();

  if (!isMobileAndLocal()) {
    await packagesStore.loadPackages();
  }
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Home
</h3>

{#if Object.keys(packagesStore.packages).length > 0}
  {#each Object.entries(packagesStore.packages).sort(([a], [b]) => a.localeCompare(b)) as [name, p]}
    <Card.Root>
      <Card.Header>
        <Card.Title>{name}</Card.Title>
        <Card.Description>
          {p.description}
        </Card.Description>
      </Card.Header>
      <Card.Content>
        <Button onclick={() => selectPackage(p)}>
          Select
        </Button>
      </Card.Content>
    </Card.Root>
  {/each}
{:else}
  <p>No packages available.</p>
{/if}
