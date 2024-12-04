<script lang="ts">
import { onMount, onDestroy } from "svelte";
import { invoke } from "@tauri-apps/api/core";
import type { Package } from "$lib/types";
import { Button } from "$lib/components/ui/button";
import * as Card from "$lib/components/ui/card/index.js";
import { platform } from "@tauri-apps/plugin-os";
import { serverUrlStore } from "$stores/serverUrl.svelte";
import { systemInfoStore } from "$stores/systemInfo.svelte";
import { dockerStatus } from "$stores/dockerStatus.svelte";
import { selectedPackageStore } from "$stores/selectedPackage.svelte";
import { goto } from "$app/navigation";
import { error } from "$utils/error";
import Link from "$lib/components/ui/link/link.svelte";

let packages: { [name: string]: Package } = $state({});

async function loadPackages() {
  try {
    packages = await invoke("get_packages");
  } catch (e) {
    error(`Failed to load packages: ${e}`);
  }
}

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
  // prefetch stores async
  if (!systemInfoStore.systemInfo) systemInfoStore.fetchSystemInfo();

  // do not fetch if mobile and local
  if (!isMobileAndLocal()) {
    await loadPackages();
  }
});
</script>

<h3 class="scroll-m-20 text-2xl font-semibold tracking-tight mb-4">
  Home
</h3>

{#if Object.keys(packages).length > 0}
  {#each Object.entries(packages).sort( ([a], [b]) => a.localeCompare(b), ) as [name, p]}
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
